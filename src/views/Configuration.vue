<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Button from "primevue/button";
import Card from "primevue/card";
import Drawer from "primevue/drawer";
import InputNumber from "primevue/inputnumber";
import Message from "primevue/message";
import Select from "primevue/select";
import Slider from "primevue/slider";
import Tag from "primevue/tag";
import { computed, onMounted, ref } from "vue";
import { CODECS, RESOLUTIONS, useAppStore } from "../stores/app";

interface DrawerSection {
  title: string;
  description: string;
}

const store = useAppStore();

const drawer = ref<DrawerSection | null>(null);
const drawerShow = ref(false);
const drawers: Record<string, DrawerSection> = {
  crf: {
    title: "Quality (CRF)",
    description: `Constant Rate Factor (CRF) is a video encoding mode that maintains a consistent,
      user-defined quality level across an entire video by varying the bitrate (file size)
      based on scene complexity. Lower CRF values yield higher quality and larger files,
      with typical scales ranging from 0–51. Default value is 23 (good balance).`,
  },
};

onMounted(async () => {
  if (store.ffmpegVersion || store.ffmpegError) {
    return;
  }

  try {
    const [v] = await Promise.all([
      invoke<string>("get_ffmpeg_version"),
      new Promise((resolve) => setTimeout(resolve, 3000)),
    ]);
    store.ffmpegVersion = v;
  } catch (e: unknown) {
    store.ffmpegError = String(e);
  }
});

const qualityTagLabel = computed(() => {
  const q = store.output.quality;
  if (q === 0) return "Lossless";
  if (q <= 18) return "High";
  if (q <= 28) return "Medium";
  if (q <= 36) return "Low";
  return "Very Low";
});

const qualityTagSeverity = computed(() => {
  const q = store.output.quality;
  if (q <= 18) return "success";
  if (q <= 28) return "info";
  if (q <= 36) return "warn";
  return "danger";
});
</script>

<template>
  <div>
    <div class="flex flex-col gap-4">
      <Message v-if="store.ffmpegError" severity="error">
        FFmpeg not found — install it and ensure it is on your PATH
      </Message>
      <Message v-else-if="store.ffmpegVersion" severity="success">
        FFmpeg with version {{ store.ffmpegVersion }} found
      </Message>
      <Message v-else>Getting FFmpeg version...</Message>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card>
          <template #title>Video Format</template>
          <template #content>
            <div class="flex flex-col gap-3">
              <div class="flex flex-col">
                <label>Resolution</label>
                <Select
                  v-model="store.output.resolution"
                  :options="RESOLUTIONS"
                  optionLabel="label"
                  checkmark
                  fluid
                  :highlightOnSelect="false"
                />
              </div>
              <div class="flex flex-col">
                <label>Codec</label>
                <Select
                  v-model="store.output.codec"
                  :options="CODECS"
                  optionLabel="label"
                  checkmark
                  fluid
                  :highlightOnSelect="false"
                />
              </div>
              <div class="flex flex-col">
                <label>FPS</label>
                <Select
                  v-model="store.output.fps"
                  :options="[24, 25, 30, 50, 60, 75, 90, 120, 144]"
                  checkmark
                  fluid
                  :highlightOnSelect="false"
                />
              </div>
              <div class="flex flex-col">
                <div class="flex items-center justify-center mb-1">
                  <label class="mr-1">Quality (CRF)</label>
                  <Button
                    icon="pi pi-info-circle"
                    variant="text"
                    severity="secondary"
                    rounded
                    class="w-8! h-8!"
                    @click="
                      drawer = drawers.crf;
                      drawerShow = true;
                    "
                  />
                  <Tag class="ml-auto h-6!" :severity="qualityTagSeverity" rounded>{{ qualityTagLabel }}</Tag>
                </div>
                <InputNumber v-model="store.output.quality" mode="decimal" showButtons fluid :min="0" :max="51" />
                <div class="inline-flex items-center justify-center px-3 pt-2 gap-5">
                  <span>0</span>
                  <div class="flex flex-col w-full">
                    <Slider v-model="store.output.quality" :min="0" :max="51" />
                  </div>
                  <span>51</span>
                </div>
              </div>
            </div>
          </template>
        </Card>
        <Card>
          <template #title>Time Configuration</template>
          <template #content>
            <div class="flex flex-col gap-3">
              <div class="flex flex-col">
                <label>Duration</label>
                <InputNumber
                  v-model="store.output.duration"
                  mode="decimal"
                  suffix=" second(s)"
                  showButtons
                  fluid
                  :min="1"
                />
              </div>
            </div>
          </template>
        </Card>
      </div>
    </div>

    <Drawer v-model:visible="drawerShow" :header="drawer?.title">
      <p>{{ drawer?.description }}</p>
    </Drawer>
  </div>
</template>
