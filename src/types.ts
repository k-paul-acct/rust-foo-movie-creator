export interface Resolution {
  width: number;
  height: number;
  label: string;
}

export interface Codec {
  value: string;
  label: string;
}

export interface OutputConfig {
  outputPath: string;
  codec: Codec;
  resolution: Resolution;
  fps: number;
  quality: number;
  transition: TransitionType;
  duration: number;
}

export type TransitionType =
  | "none"
  | "fade"
  | "wipeleft"
  | "wiperight"
  | "slideleft"
  | "slideup"
  | "dissolve"
  | "pixelize";

export interface ImageEntry {
  path: string;
  name: string;
  selected: boolean;
}

export interface ImagesConfig {
  sourceDir: string | null;
  images: ImageEntry[];
  selectionMode: "directory" | "manual";
  count: number | null;
  shuffle: boolean;
}

export type EffectName =
  | "zoom_in"
  | "zoom_out"
  | "pan_left"
  | "pan_right"
  | "pan_up"
  | "pan_down"
  | "zoom_in_pan_right"
  | "zoom_in_pan_left"
  | "zoom_out_pan_right"
  | "zoom_out_pan_left"
  // | "rotate_zoom"
  | "ken_burns";

export interface EffectEntry {
  name: EffectName;
  label: string;
  enabled: boolean;
}

export interface EffectsConfig {
  enabledEffects: EffectName[];
  minDuration: number;
  maxDuration: number;
  targetTotalDuration: number | null;
  seed: number | null;
  noRepeatConsecutive: boolean;
}

export type ShapeType = "circle" | "rectangle" | "mixed";

export interface ShapeColor {
  id: number;
  color: string;
  a: number;
}

export interface ScreensaverConfig {
  enabled: boolean;
  shapeType: ShapeType;
  shapeCount: number;
  minSize: number;
  maxSize: number;
  minSpeed: number;
  maxSpeed: number;
  backgroundColor: string;
  shapeColors: ShapeColor[];
  blurEdges: boolean;
  seed: number | null;
}

export interface ProgressPayload {
  phase: "encoding" | "done" | "error";
  currentFrame: number;
  totalFrames: number;
  percentage: number;
  message: string;
}

// TODO: Rename to AppState.
export interface ProjectState {
  output: OutputConfig;
  images: ImagesConfig;
  effects: EffectsConfig;
  screensaver: ScreensaverConfig;
}
