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
import { useAppStore } from "../stores/app";

const store = useAppStore();

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
          Generates an animated screen-saver clip (moving shapes, no bouncing) and uses it as a standalone video.
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
              <div class="flex items-center mt-1 gap-2">
                <ToggleSwitch v-model="store.screensaver.blurEdges" />
                <label>Soft-edge shapes (blur) </label>
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
                class="flex flex-col h-150 overflow-y-auto p-3 gap-2 border rounded-md border-surface-200 dark:border-surface-700"
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
              <Button icon="pi pi-refresh" label="Refresh" severity="secondary" size="small" />
            </div>
          </template>
          <template #content>
            <div class="flex items-center justify-center w-full aspect-square border border-surface-200 dark:border-surface-700 rounded-md">
              <span>JTBD</span>
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
