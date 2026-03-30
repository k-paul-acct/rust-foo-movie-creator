<script setup lang="ts">
import Badge from "primevue/badge";
import Button from "primevue/button";
import Card from "primevue/card";
import ColorPicker from "primevue/colorpicker";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import InputNumber from "primevue/inputnumber";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import Slider from "primevue/slider";
import ToggleSwitch from "primevue/toggleswitch";
import { onMounted, onUnmounted, ref, watch } from "vue";
import { useAppStore } from "../stores/app";
import { ShapeColor } from "../types";
import { rng } from "../utils/rng";

const store = useAppStore();

const canvasPreview = ref<HTMLCanvasElement | null>(null);
let animId = 0;
let lastTime = 0;

onMounted(() => {
  if (store.screensaver.enabled) startPreview();
});

onUnmounted(() => cancelAnimationFrame(animId));

watch(
  () => store.screensaver.enabled,
  (v) => {
    if (v) startPreview();
    else cancelAnimationFrame(animId);
  },
);

watch(
  () => [store.screensaver.shapeType, store.screensaver.shapeCount, store.screensaver.seed],
  () => {
    if (store.screensaver.enabled) buildShapes();
  },
);

watch(
  () => [store.screensaver.minSize, store.screensaver.maxSize, store.screensaver.minSpeed, store.screensaver.maxSpeed],
  () => {
    if (store.screensaver.enabled) updateShapesSizeAndSpeed();
  },
);

watch(
  () => [store.screensaver.shapeColors],
  () => {
    if (store.screensaver.enabled) updateShapesColors();
  },
);

interface AnimShape {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  horizontalOrientation: boolean;
  color: ShapeColor;
  isCircle: boolean;
}

let shapesGenSeed = 0;
let shapes: AnimShape[] = [];

function updateShapesSizeAndSpeed() {
  const cfg = store.screensaver;
  const nextRand = rng(shapesGenSeed);

  for (const sh of shapes) {
    const size = (cfg.minSize + nextRand() * (cfg.maxSize - cfg.minSize)) / 100;
    const speed = (cfg.minSpeed + nextRand() * (cfg.maxSpeed - cfg.minSpeed)) / 100;
    const angle = nextRand() * 2 * Math.PI;

    sh.size = size;
    sh.vx = Math.cos(angle) * speed;
    sh.vy = Math.sin(angle) * speed;

    for (let i = 0; i < 5; ++i) {
      nextRand();
    }
  }
}

function updateShapesColors() {
  const cfg = store.screensaver;

  for (const sh of shapes) {
    if (cfg.shapeColors.indexOf(sh.color) === -1) {
      fullUpdate();
      return;
    }
  }

  function fullUpdate() {
    const nextRand = rng(shapesGenSeed);
    for (const sh of shapes) {
      for (let i = 0; i < 6; ++i) {
        nextRand();
      }

      const color = cfg.shapeColors[Math.floor(nextRand() * cfg.shapeColors.length)];
      sh.color = color;

      nextRand();
    }
  }
}

function buildShapes() {
  const cfg = store.screensaver;
  shapesGenSeed = cfg.seed ?? (Math.random() * 0x100000000) | 0;
  const nextRand = rng(shapesGenSeed);

  shapes = Array.from({ length: cfg.shapeCount }, () => {
    const size = (cfg.minSize + nextRand() * (cfg.maxSize - cfg.minSize)) / 100;
    const speed = (cfg.minSpeed + nextRand() * (cfg.maxSpeed - cfg.minSpeed)) / 100;
    const angle = nextRand() * 2 * Math.PI;
    return {
      x: nextRand(),
      y: nextRand(),
      vx: Math.cos(angle) * speed,
      vy: Math.sin(angle) * speed,
      size,
      horizontalOrientation: nextRand() < 0.5,
      color: cfg.shapeColors[Math.floor(nextRand() * cfg.shapeColors.length)],
      isCircle:
        cfg.shapeType === "circle" ? nextRand() < 1 : cfg.shapeType === "rectangle" ? nextRand() < 0 : nextRand() < 0.5,
    };
  });
}

function drawFrame(t: number) {
  const canvas = canvasPreview.value;
  if (!canvas) return;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const displayWidth = canvas.clientWidth;
  const displayHeight = canvas.clientHeight;
  const dpr = window.devicePixelRatio || 1;
  const W = Math.floor(displayWidth * dpr);
  const H = Math.floor(displayHeight * dpr);

  if (canvas.width !== W || canvas.height !== H) {
    canvas.width = W;
    canvas.height = H;
  }

  const bg = store.screensaver.backgroundColor;
  ctx.fillStyle = "#" + bg;
  ctx.fillRect(0, 0, W, H);

  for (const sh of shapes) {
    const alpha = Math.round(sh.color.a * 2.55);
    const alphaHex = alpha.toString(16).padStart(2, "0");
    ctx.fillStyle = `#${sh.color.color}${alphaHex}`;

    const unit = Math.min(W, H);
    const size = sh.size * unit;
    const [w, h] = sh.isCircle ? [size, size] : sh.horizontalOrientation ? [size, size * (2 / 3)] : [size * (2 / 3), size];

    let dx = sh.vx * t * unit;
    let dy = sh.vy * t * unit;
    let x = sh.x * W + dx;
    let y = sh.y * H + dy;

    if (x + w / 2 > W) {
      x = W - w / 2;
      sh.vx *= -1;
    } else if (x - w / 2 < 0) {
      x = w / 2;
      sh.vx *= -1;
    }

    if (y + h / 2 > H) {
      y = H - h / 2;
      sh.vy *= -1;
    } else if (y - h / 2 < 0) {
      y = h / 2;
      sh.vy *= -1;
    }

    sh.x = x / W;
    sh.y = y / H;

    ctx.beginPath();

    if (sh.isCircle) {
      ctx.arc(x, y, w / 2, 0, 2 * Math.PI);
    } else {
      ctx.rect(x - w / 2, y - h / 2, w, h);
    }

    ctx.fill();
  }
}

function animate(currentTime: number) {
  if (!lastTime) lastTime = currentTime;
  const deltaTime = (currentTime - lastTime) / 1000;
  lastTime = currentTime;
  drawFrame(deltaTime);
  animId = requestAnimationFrame(animate);
}

function startPreview() {
  buildShapes();
  lastTime = 0;
  cancelAnimationFrame(animId);
  animId = requestAnimationFrame(animate);
}

const shapeTypes = [
  { value: "mixed", label: "Mixed" },
  { value: "circle", label: "Circle" },
  { value: "rectangle", label: "Rectangle" },
];

function padHex(color: string) {
  return color.length != 6 ? color.padStart(6, "0") : color;
}

function addColor() {
  const alphabet = "0123456789abcdef";
  const id = Math.random();
  const color = Array.from({ length: 6 }, () => alphabet[Math.floor(Math.random() * 16)]).join("");
  const a = 75;
  store.screensaver.shapeColors.unshift({ id, color, a });
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <Card>
      <template #title>
        <div class="flex items-center gap-2">
          <ToggleSwitch v-model="store.screensaver.enabled" />
          <span>Use Screensaver</span>
        </div>
      </template>
      <template #content>
        <span>
          Generates an animated screen-saver clip (moving shapes with bouncing) and uses it as a standalone video.
        </span>
      </template>
    </Card>
    <Transition name="toggle-slide">
      <div v-if="store.screensaver.enabled" class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card>
          <template #title>Shape Settings</template>
          <template #content>
            <div class="flex flex-col gap-3">
              <div class="flex flex-col">
                <label>Shape Type</label>
                <Select
                  v-model="store.screensaver.shapeType"
                  :options="shapeTypes"
                  optionLabel="label"
                  optionValue="value"
                  checkmark
                  fluid
                  :highlightOnSelect="false"
                />
              </div>
              <div class="flex flex-col">
                <div class="flex justify-between">
                  <label>Shape Count</label>
                  <label>{{ store.screensaver.shapeCount }}</label>
                </div>
                <div
                  class="inline-flex items-center justify-center bg-surface-100 dark:bg-surface-800 rounded-md px-3 py-2 gap-5"
                >
                  <span>1</span>
                  <div class="flex flex-col w-full">
                    <Slider v-model="store.screensaver.shapeCount" :min="1" :max="100" />
                  </div>
                  <span>100</span>
                </div>
              </div>
              <div
                class="grid grid-cols-1 md:grid-cols-1 lg:grid-cols-2 p-3 gap-4 border border-surface-200 dark:border-surface-700 rounded-md"
              >
                <div class="flex flex-col">
                  <div class="flex justify-between">
                    <label>Min Size</label>
                    <label>{{ store.screensaver.minSize }}%</label>
                  </div>
                  <div
                    class="inline-flex items-center justify-center bg-surface-100 dark:bg-surface-800 rounded-md px-3 py-2 gap-5"
                  >
                    <span>1%</span>
                    <div class="flex flex-col w-full">
                      <Slider v-model="store.screensaver.minSize" :min="1" :max="100" />
                    </div>
                    <span>100%</span>
                  </div>
                </div>
                <div class="flex flex-col">
                  <div class="flex justify-between">
                    <label>Max Size</label>
                    <label>{{ store.screensaver.maxSize }}%</label>
                  </div>
                  <div
                    class="inline-flex items-center justify-center bg-surface-100 dark:bg-surface-800 rounded-md px-3 py-2 gap-5"
                  >
                    <span>1%</span>
                    <div class="flex flex-col w-full">
                      <Slider v-model="store.screensaver.maxSize" :min="1" :max="100" />
                    </div>
                    <span>100%</span>
                  </div>
                </div>
              </div>
              <div
                class="grid grid-cols-1 md:grid-cols-1 lg:grid-cols-2 p-3 gap-4 border border-surface-200 dark:border-surface-700 rounded-md"
              >
                <div class="flex flex-col">
                  <div class="flex justify-between">
                    <label>Min Speed</label>
                    <label>{{ store.screensaver.minSpeed }}% per second</label>
                  </div>
                  <div
                    class="inline-flex items-center justify-center bg-surface-100 dark:bg-surface-800 rounded-md px-3 py-2 gap-5"
                  >
                    <span>1%</span>
                    <div class="flex flex-col w-full">
                      <Slider v-model="store.screensaver.minSpeed" :min="1" :max="100" />
                    </div>
                    <span>100%</span>
                  </div>
                </div>
                <div class="flex flex-col">
                  <div class="flex justify-between">
                    <label>Max Speed</label>
                    <label>{{ store.screensaver.maxSpeed }}% per second</label>
                  </div>
                  <div
                    class="inline-flex items-center justify-center bg-surface-100 dark:bg-surface-800 rounded-md px-3 py-2 gap-5"
                  >
                    <span>1%</span>
                    <div class="flex flex-col w-full">
                      <Slider v-model="store.screensaver.maxSpeed" :min="1" :max="100" />
                    </div>
                    <span>100%</span>
                  </div>
                </div>
              </div>
            </div>
          </template>
        </Card>
        <Card>
          <template #title>Colors</template>
          <template #content>
            <div class="flex flex-col gap-3">
              <div class="flex flex-col">
                <label>Background Color</label>
                <div class="flex gap-1">
                  <ColorPicker
                    :pt="{
                      preview: {
                        class: 'h-full! w-6! rounded-md!',
                      },
                    }"
                    v-model="store.screensaver.backgroundColor"
                    format="hex"
                  />
                  <InputGroup>
                    <InputGroupAddon>
                      <i class="pi pi-hashtag" />
                    </InputGroupAddon>
                    <InputText
                      v-model="store.screensaver.backgroundColor"
                      v-keyfilter.hex
                      maxlength="6"
                      @blur="() => (store.screensaver.backgroundColor = padHex(store.screensaver.backgroundColor))"
                    />
                  </InputGroup>
                </div>
              </div>
              <div
                class="flex flex-col max-h-150 overflow-x-hidden overflow-y-auto p-3 gap-2 border rounded-md border-surface-200 dark:border-surface-700"
              >
                <div class="flex items-center justify-between">
                  <div class="flex gap-2">
                    <label>Shape Colors</label>
                    <Badge :value="store.screensaver.shapeColors.length" severity="secondary" />
                  </div>
                  <Button icon="pi pi-plus" label="Add" severity="secondary" size="small" @click="addColor" />
                </div>
                <TransitionGroup name="list" tag="ul" class="flex flex-col gap-3 relative">
                  <li v-for="shapeColor in store.screensaver.shapeColors" :key="shapeColor.id">
                    <div class="flex p-3 gap-4 rounded-md bg-surface-100 dark:bg-surface-800">
                      <div class="grid auto-rows-fr grid-cols-1 lg:grid-cols-2 w-full gap-4">
                        <div class="flex flex-col w-full gap-1">
                          <span class="text-sm">Color</span>
                          <div class="flex gap-1">
                            <ColorPicker
                              :pt="{
                                preview: {
                                  class: 'h-full! w-6! rounded-md!',
                                },
                              }"
                              v-model="shapeColor.color"
                              format="hex"
                            />
                            <InputGroup>
                              <InputGroupAddon>
                                <i class="pi pi-hashtag" />
                              </InputGroupAddon>
                              <InputText
                                v-model="shapeColor.color"
                                v-keyfilter.hex
                                maxlength="6"
                                @blur="() => (shapeColor.color = padHex(shapeColor.color))"
                              />
                            </InputGroup>
                          </div>
                        </div>
                        <div class="flex flex-col w-full gap-1">
                          <div class="flex items-center justify-between w-full">
                            <span class="text-sm">Opacity</span>
                            <span class="text-sm">{{ shapeColor.a }}%</span>
                          </div>
                          <div
                            class="flex flex-col justify-center w-full h-full px-5 border rounded-md border-surface-200 dark:border-surface-700"
                          >
                            <Slider v-model="shapeColor.a" :min="1" :max="100" />
                          </div>
                        </div>
                      </div>
                      <div class="flex">
                        <Button
                          icon="pi pi-times"
                          severity="danger"
                          variant="outlined"
                          @click="
                            store.screensaver.shapeColors = store.screensaver.shapeColors.filter((c) => c != shapeColor)
                          "
                          :disabled="store.screensaver.shapeColors.length <= 1"
                        />
                      </div>
                    </div>
                  </li>
                </TransitionGroup>
              </div>
            </div>
          </template>
        </Card>
        <Card>
          <template #title>Seed</template>
          <template #content>
            <InputNumber
              v-model="store.screensaver.seed"
              mode="decimal"
              placeholder="Random Seed"
              showButtons
              fluid
              :min="-2147483648"
              :max="2147483647"
            />
          </template>
        </Card>
        <Card>
          <template #title>
            <div class="flex items-center justify-between">
              <span>Preview</span>
              <Button @click="startPreview" icon="pi pi-refresh" label="Refresh" severity="secondary" size="small" />
            </div>
          </template>
          <template #content>
            <div class="flex overflow-hidden border border-surface-200 dark:border-surface-700 rounded-md">
              <canvas ref="canvasPreview" class="w-full aspect-square md:aspect-2/1" />
            </div>
          </template>
        </Card>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.toggle-slide-enter-active,
.toggle-slide-leave-active {
  transition:
    opacity var(--transition),
    transform var(--transition);
}
.toggle-slide-enter-from,
.toggle-slide-leave-to {
  opacity: 0;
  transform: translateY(6px);
}

.list-move,
.list-enter-active,
.list-leave-active {
  transition:
    opacity var(--transition),
    transform var(--transition);
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(24px);
}
.list-leave-active {
  position: absolute;
}
</style>
