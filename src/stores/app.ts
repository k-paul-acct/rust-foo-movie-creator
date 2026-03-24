import { defineStore } from "pinia";
import { computed, ref, watch } from "vue";
import type { EffectEntry, EffectsConfig, ImagesConfig, OutputConfig, ScreensaverConfig } from "../types";

export const EFFECT_ENTRIES: EffectEntry[] = [
  { name: "zoom_in", label: "Zoom In", enabled: true },
  { name: "zoom_out", label: "Zoom Out", enabled: true },
  { name: "pan_left", label: "Pan Left", enabled: true },
  { name: "pan_right", label: "Pan Right", enabled: true },
  { name: "pan_up", label: "Pan Up", enabled: true },
  { name: "pan_down", label: "Pan Down", enabled: true },
  { name: "zoom_in_pan_right", label: "Zoom In + Pan Right", enabled: true },
  { name: "zoom_in_pan_left", label: "Zoom In + Pan Left", enabled: true },
  { name: "zoom_out_pan_right", label: "Zoom Out + Pan Right", enabled: true },
  { name: "zoom_out_pan_left", label: "Zoom Out + Pan Left", enabled: true },
  // { name: "rotate_zoom", label: "Rotate & Zoom", enabled: true },
  { name: "ken_burns", label: "Ken Burns", enabled: true },
];

export const RESOLUTIONS = [
  { width: 3840, height: 2160, label: "4K UHD (3840×2160)" },
  { width: 2560, height: 1440, label: "QHD (2560×1440)" },
  { width: 1920, height: 1080, label: "Full HD (1920×1080)" },
  { width: 1280, height: 720, label: "HD (1280×720)" },
  { width: 854, height: 480, label: "FWVGA (854×480)" },
  { width: 640, height: 360, label: "nHD (640×360)" },
];

export const CODECS = [
  { value: "h264", label: "H.264/AVC — widely supported" },
  { value: "hevc", label: "H.265/HEVC — improved compression" },
  { value: "vp8", label: "VP8 — open, web-optimized" },
  { value: "vp9", label: "VP9 — open, web-optimized" },
  { value: "av1", label: "AV1 — open, high quality and compression" },
  // { value: "av2", label: "AV2 — open, high quality and compression" },
];

export const TRANSITIONS = [
  { value: "none", label: "None" },
  { value: "fade", label: "Fade" },
  { value: "wipeleft", label: "Wipe Left" },
  { value: "wiperight", label: "Wipe Right" },
  { value: "slideleft", label: "Slide Left" },
  { value: "slideup", label: "Slide Up" },
  { value: "dissolve", label: "Dissolve" },
  { value: "pixelize", label: "Pixelize" },
];

export const useSettingsStore = defineStore("settings", {
  state: () => ({ darkMode: false }),
  persist: true,
});

export const useAppStore = defineStore("app", () => {
  const settings = ref({
    appName: "Foo Movie",
    language: "en",
  });

  const sidebarCollapsed = ref(false);

  const ffmpegVersion = ref<string | null>();
  const ffmpegError = ref<string | null>();

  const output = ref<OutputConfig>({
    outputPath: "",
    codec: CODECS[0],
    resolution: RESOLUTIONS[2],
    fps: 30,
    quality: 23,
    transition: "fade",
    duration: 30,
  });

  const images = ref<ImagesConfig>({
    sourceDir: null,
    images: [],
    selectionMode: "directory",
    count: null,
    shuffle: true,
  });

  const selectedImagesCount = computed(() => images.value.images.filter((i) => i.selected).length);

  const effects = ref<EffectsConfig>({
    enabledEffects: EFFECT_ENTRIES.map((e) => e.name),
    minDuration: 3.0,
    maxDuration: 7.0,
    targetTotalDuration: null,
    seed: null,
    noRepeatConsecutive: true,
  });

  const screensaver = ref<ScreensaverConfig>({
    enabled: false,
    shapeType: "mixed",
    shapeCount: 10,
    minSize: 25,
    maxSize: 75,
    minSpeed: 25,
    maxSpeed: 75,
    backgroundColor: "202020",
    shapeColors: [
      { id: -1, color: "ff746c", a: 75 },
      { id: -2, color: "80ef80", a: 75 },
      { id: -3, color: "b3ebf2", a: 75 },
      { id: -4, color: "ffee8c", a: 75 },
      { id: -5, color: "b39eb5", a: 75 },
    ],
    blurEdges: true,
    seed: null,
  });

  watch(
    () => screensaver.value.minSize,
    (newMin) => {
      if (newMin > screensaver.value.maxSize) {
        screensaver.value.maxSize = newMin;
      }
    },
  );

  watch(
    () => screensaver.value.maxSize,
    (newMax) => {
      if (newMax < screensaver.value.minSize) {
        screensaver.value.minSize = newMax;
      }
    },
  );

  watch(
    () => screensaver.value.minSpeed,
    (newMin) => {
      if (newMin > screensaver.value.maxSpeed) {
        screensaver.value.maxSpeed = newMin;
      }
    },
  );

  watch(
    () => screensaver.value.maxSpeed,
    (newMax) => {
      if (newMax < screensaver.value.minSpeed) {
        screensaver.value.minSpeed = newMax;
      }
    },
  );

  function validate(): string[] {
    const errs: string[] = [];
    if (!output.value.outputPath) errs.push("No output path set.");
    const activeImages = images.value.images.filter((i) => i.selected);
    if (activeImages.length === 0 && !screensaver.value.enabled) errs.push("No images selected.");
    if (effects.value.enabledEffects.length === 0 && activeImages.length > 0) errs.push("No effects enabled.");
    return errs;
  }

  return {
    settings,
    sidebarCollapsed,
    ffmpegVersion,
    ffmpegError,
    output,
    images,
    selectedImagesCount,
    effects,
    screensaver,
    validate,
  };
});
