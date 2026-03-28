use anyhow::{Context, Result, bail};
use rand::{Rng, RngExt, SeedableRng, random, seq::IndexedRandom};
use rand_xoshiro::Xoshiro128StarStar;
use regex::{Regex, RegexBuilder};
use std::{process::Stdio, sync::LazyLock};
use tiny_skia::*;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    pin,
    process::{Child, ChildStdin, Command},
};
use tokio_util::sync::CancellationToken;

use crate::commands::{GenerateConfig, ScreensaverConfig, ScreensaverShapeColor};

pub struct ProgressReporter {
    phase: String,
    message: String,
    percentage: u32,
    callback: Box<dyn Fn(&str, u32, &str) + Send + Sync>,
}

impl ProgressReporter {
    pub fn from_callback(callback: Box<dyn Fn(&str, u32, &str) + Send + Sync>) -> ProgressReporter {
        ProgressReporter {
            phase: "generating".to_string(),
            message: "Starting FFmpeg...".to_string(),
            percentage: 0,
            callback: callback,
        }
    }

    pub fn enter_stage(&mut self, stage: &str, message: &str) {
        self.phase = stage.to_string();
        self.message = message.to_string();
        (self.callback)(&self.phase, self.percentage, &self.message);
    }

    pub fn update_percentage(&mut self, p: u32) {
        self.percentage = p;
        (self.callback)(&self.phase, self.percentage, &self.message);
    }
}

struct AnimShape {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    dx: f32,
    dy: f32,
    path: Path,
    paint: Paint<'static>,
}

impl AnimShape {
    #[inline]
    fn update(&mut self, dt: f32, width: f32, height: f32) {
        self.x += self.dx * dt;
        self.y += self.dy * dt;

        if self.x < 0.0 {
            self.x = 0.0;
            self.dx *= -1.0;
        } else if self.x + self.w > width {
            self.x = width - self.w;
            self.dx *= -1.0;
        }

        if self.y < 0.0 {
            self.y = 0.0;
            self.dy *= -1.0;
        } else if self.y + self.h > height {
            self.y = height - self.h;
            self.dy *= -1.0;
        }
    }

    #[inline]
    fn draw(&self, canvas: &mut Pixmap) {
        let ts = Transform::from_translate(self.x, self.y);
        canvas.fill_path(&self.path, &self.paint, FillRule::Winding, ts, None);
    }
}

impl Into<Color> for ScreensaverShapeColor {
    fn into(self) -> Color {
        fn color_from_hex(hex: &str, alpha: f32) -> Option<Color> {
            let hex = hex.strip_prefix('#').unwrap_or(hex);
            if hex.len() != 6 {
                None
            } else {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = (alpha * 255.0).round() as u8;
                Some(Color::from_rgba8(r, g, b, a))
            }
        }

        let alpha = self.alpha as f32 / 100.0;
        color_from_hex(&self.color, alpha).unwrap()
    }
}

pub async fn run_generation(
    cfg: &GenerateConfig,
    progress: &mut ProgressReporter,
    ct: CancellationToken,
) -> Result<()> {
    let ffmpeg = find_ffmpeg().await?;

    let seed = cfg.seed.unwrap_or_else(|| random());
    let mut rng = Xoshiro128StarStar::seed_from_u64(seed as u64);

    let total_frames = (cfg.fps as f32 * cfg.duration) as u32;

    let mut ffmpeg_child = spawn_ffmpeg(
        &ffmpeg,
        cfg.width,
        cfg.height,
        cfg.fps,
        &cfg.codec,
        cfg.quality,
        &cfg.output_path,
    )?;

    let stdin = ffmpeg_child
        .stdin
        .take()
        .context("Could not open FFmpeg stdin")?;

    pipe_frames(stdin, cfg, &mut rng, total_frames, progress, ct.clone()).await?;

    if ct.is_cancelled() {
        bail!("Cancelled.");
    }

    progress.enter_stage("encoding", "Encoding a video...");

    tokio::select! {
        proc_result = ffmpeg_child.wait() => {
            let status = proc_result.context("FFmpeg did not finish")?;

            if !status.success() {
                bail!("FFmpeg exited with status: {}", status);
            }

            progress.enter_stage("done", "Done!");

            Ok(())
        },
        _ = ct.cancelled() => {
            let _ = ffmpeg_child.kill().await;
            bail!("Cancelled.");
        }
    }
}

pub async fn find_ffmpeg() -> Result<String> {
    for candidate in &["ffmpeg", "ffmpeg.exe"] {
        if Command::new(candidate)
            .arg("-version")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map_or(false, |status| status.success())
        {
            return Ok(candidate.to_string());
        }
    }
    bail!("FFmpeg not found in PATH. Please install FFmpeg.");
}

pub async fn ffmpeg_version() -> Result<String> {
    static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        RegexBuilder::new(
            r"V?(\d+(?:\.\d+){0,2}(?:-[\dA-Z]+[\-\.\dA-Z]*)?(?:\+[\dA-Z]+[\-\.\dA-Z]*)?)",
        )
        .case_insensitive(true)
        .build()
        .unwrap()
    });

    let bin = find_ffmpeg().await?;
    let out = Command::new(&bin)
        .arg("-version")
        .output()
        .await
        .with_context(|| format!("Failed to run `{} -version`", bin))?;
    let output = std::str::from_utf8(&out.stdout)
        .with_context(|| format!("Could not read output of `{} -version`", bin))?;
    Ok(output
        .lines()
        .next()
        .and_then(|line| VERSION_REGEX.captures(line))
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "<unknown>".to_string()))
}

fn spawn_ffmpeg(
    ffmpeg: &str,
    width: u32,
    height: u32,
    fps: u32,
    codec: &str,
    quality: u32,
    output: &str,
) -> Result<Child> {
    let mut cmd = Command::new(ffmpeg);

    cmd //
        .args(["-y"]) // overwrite output
        .args(["-f", "rawvideo"]) // input format
        .args(["-pixel_format", "rgba"]) // rgba pixels
        .args(["-video_size", &format!("{width}x{height}")]) // resolution
        .args(["-framerate", &fps.to_string()]) // fps
        .args(["-i", "pipe:0"]) // read from stdin
        .args(["-c:v", codec]) // codec
        .args(["-pix_fmt", "yuv420p"]) // pixel format
        .args(["-movflags", "+faststart"]);

    if codec == "h264" || codec == "hevc" {
        cmd.args(["-crf", &quality.to_string()]); // quality
    }

    cmd //
        .arg(output) // output
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let child = cmd.spawn().context("Failed to spawn FFmpeg")?;

    Ok(child)
}

async fn pipe_frames<R: Rng>(
    mut stdin: ChildStdin,
    cfg: &GenerateConfig,
    rng: &mut R,
    total_frames: u32,
    progress: &mut ProgressReporter,
    ct: CancellationToken,
) -> Result<()> {
    if let Some(ss) = &cfg.screensaver {
        progress.enter_stage("generating", "Rendering screensaver...");
        render_screensaver_frames(&mut stdin, cfg, rng, ss, total_frames, progress, ct.clone())
            .await?;
    }

    Ok(())
}

async fn render_screensaver_frames<R: Rng, W: AsyncWrite + Unpin>(
    stdin: &mut W,
    cfg: &GenerateConfig,
    rng: &mut R,
    ss: &ScreensaverConfig,
    ss_frames: u32,
    progress: &mut ProgressReporter,
    ct: CancellationToken,
) -> Result<()> {
    let report_every = ((ss_frames as f32 / 100.0) as u32).max(1);
    let mut shapes: Vec<AnimShape> = (0..ss.shape_count)
        .map(|_| {
            let color = ss
                .colors
                .choose(rng)
                .cloned()
                .unwrap_or_else(|| ScreensaverShapeColor {
                    color: "ff746c".to_string(),
                    alpha: 75,
                })
                .into();
            let is_circle = match ss.shape_type.as_str() {
                "circle" => true,
                "rectangle" => false,
                _ => rng.random_bool(0.5),
            };
            let unit = cfg.width.min(cfg.height) as f32;
            let is_horizontal = rng.random_bool(0.5);
            let size = rng.random_range(ss.min_size..=ss.max_size) as f32 / 100.0 * unit;
            let (w, h) = if is_circle {
                (size, size)
            } else if is_horizontal {
                (size, size * (2.0 / 3.0))
            } else {
                (size * (2.0 / 3.0), size)
            };
            let speed = rng.random_range(ss.min_speed..=ss.max_speed) as f32 / 100.0 * unit;
            let angle = rng.random::<f32>() * std::f32::consts::TAU;
            let (sin, cos) = angle.sin_cos();
            let rect = Rect::from_xywh(0.0, 0.0, w, h).unwrap();
            AnimShape {
                x: rng.random::<f32>() * cfg.width as f32,
                y: rng.random::<f32>() * cfg.height as f32,
                w: w,
                h: h,
                dx: cos * speed,
                dy: sin * speed,
                paint: {
                    let mut paint = Paint::default();
                    paint.set_color(color);
                    paint.anti_alias = true;
                    paint
                },
                path: {
                    if is_circle {
                        PathBuilder::from_oval(rect).unwrap()
                    } else {
                        PathBuilder::from_rect(rect)
                    }
                },
            }
        })
        .collect();

    let dt = 1.0 / cfg.fps as f32;
    let background = ScreensaverShapeColor {
        color: ss.bg_color.clone(),
        alpha: 100,
    }
    .into();
    let mut canvas = Pixmap::new(cfg.width, cfg.height).context("OOM")?;

    pin!(stdin);

    for frame_idx in 0..ss_frames {
        canvas.fill(background);

        for s in &mut shapes {
            s.update(dt, cfg.width as f32, cfg.height as f32);
            s.draw(&mut canvas);
        }

        stdin.write_all(canvas.data()).await?;

        if frame_idx % report_every == 0 {
            if ct.is_cancelled() {
                bail!("Cancelled.");
            }

            let pct = (frame_idx as f32 / ss_frames as f32 * 100.0).round() as u32;
            progress.update_percentage(pct);
        }
    }

    progress.update_percentage(100);

    Ok(())
}
