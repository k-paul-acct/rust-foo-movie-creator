<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { save } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import ConfirmDialog from "primevue/confirmdialog";
import Tag from "primevue/tag";
import Toast from "primevue/toast";
import { computed, ref } from "vue";
import { RouterLink, RouterView, useRoute } from "vue-router";
import { useAppStore, useSettingsStore } from "./stores/app";
import type { ProgressPayload } from "./types";

const route = useRoute();
const store = useAppStore();
const settingsStore = useSettingsStore();

document.documentElement.classList.toggle("dark-mode", settingsStore.darkMode);

const navItems = [
  { path: "/configuration", label: "Configuration", icon: "pi-cog" },
  { path: "/screensaver", label: "Screensaver", icon: "pi-desktop", toggleBadge: () => store.screensaver.enabled },
];

const pageTitle = computed(() => {
  const titles: Record<string, string> = {
    "/configuration": "Configuration",
    "/screensaver": "Screensaver",
  };
  return titles[route.path] ?? store.settings.appName;
});

function toggleDark() {
  settingsStore.darkMode = !settingsStore.darkMode;
  document.documentElement.classList.toggle("dark-mode", settingsStore.darkMode);
}

const isGenerating = ref(false);
const progress = ref<ProgressPayload>({
  phase: "encoding",
  currentFrame: 0,
  totalFrames: 0,
  percentage: 0,
  message: "",
});
const errors = ref<string[]>([]);

async function pickOutputPath() {
  const path = await save({
    filters: [{ name: "Video", extensions: ["mp4", "mov", "mkv", "webm"] }],
    defaultPath: "output.mp4",
  });
  if (path) store.output.outputPath = path;
}

function closeProgress() {
  if (progress.value.phase !== "encoding") {
    isGenerating.value = false;
  }
}

async function generate() {
  errors.value = store.validate();
  if (errors.value.length) return;

  isGenerating.value = true;
  progress.value = {
    phase: "encoding",
    currentFrame: 0,
    totalFrames: 0,
    percentage: 0,
    message: "Starting...",
  };

  const unlisten = await listen<ProgressPayload>("video-progress", (event) => {
    progress.value = event.payload;
  });

  try {
    await invoke("generate_video", {
      config: {
        output_path: store.output.outputPath,
        codec: store.output.codec,
        width: store.output.resolution.width,
        height: store.output.resolution.height,
        fps: store.output.fps,
        quality: store.output.quality,
        transition: store.output.transition,
        transition_dur: store.output.duration,
        images: store.images.images.filter((i) => i.selected).map((i) => i.path),
        effects: store.effects.enabledEffects,
        min_dur: store.effects.minDuration,
        max_dur: store.effects.maxDuration,
        total_dur: store.effects.targetTotalDuration,
        seed: store.effects.seed,
        no_repeat: store.effects.noRepeatConsecutive,
        screensaver: store.screensaver.enabled
          ? {
              shape_type: store.screensaver.shapeType,
              shape_count: store.screensaver.shapeCount,
              min_size: store.screensaver.minSize,
              max_size: store.screensaver.maxSize,
              min_speed: store.screensaver.minSpeed,
              max_speed: store.screensaver.maxSpeed,
              bg_color: store.screensaver.backgroundColor,
              colors: store.screensaver.shapeColors.map((c) => [c.color, c.a]),
              blur_edges: store.screensaver.blurEdges,
              seed: store.screensaver.seed,
            }
          : null,
      },
    });
    progress.value = {
      phase: "done",
      currentFrame: 0,
      totalFrames: 0,
      percentage: 100,
      message: "Video created successfully",
    };
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e);
    progress.value = {
      phase: "error",
      currentFrame: 0,
      totalFrames: 0,
      percentage: 0,
      message: msg,
    };
  } finally {
    unlisten();
  }
}

async function cancel() {
  await invoke("cancel_generation");
}
</script>

<template>
  <div
    id="app-shell"
    class="flex h-dvh overflow-hidden select-none"
    :class="{ 'sidebar-collapsed': store.sidebarCollapsed }"
  >
    <aside class="sidebar">
      <div class="sidebar-header">
        <div class="sidebar-logo">
          <div class="logo-mark">
            <i class="pi pi-video" />
          </div>
          <span class="logo-text">{{ store.settings.appName }}</span>
        </div>
        <div class="sidebar-collapse-area">
          <Button
            :icon="store.sidebarCollapsed ? 'pi pi-arrow-right' : 'pi pi-arrow-left'"
            variant="text"
            severity="secondary"
            @click="store.sidebarCollapsed = !store.sidebarCollapsed"
          />
        </div>
      </div>
      <nav class="sidebar-nav">
        <RouterLink
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="nav-item"
          :class="{ active: route.path === item.path }"
        >
          <i :class="['pi', item.icon]" />
          <span>{{ item.label }}</span>
          <!-- <Tag
            v-if="item.badge"
            :value="item.badge()"
            rounded
            style="margin-left: auto; font-size: var(--app-fs-badge-sm)"
          />  -->
          <Tag
            v-if="item.toggleBadge"
            :severity="item.toggleBadge() ? 'success' : 'danger'"
            :value="item.toggleBadge() ? 'ON' : 'OFF'"
            rounded
            style="margin-left: auto; font-size: var(--app-fs-badge-sm)"
          />
        </RouterLink>
      </nav>
    </aside>

    <div class="main-area">
      <header class="topbar">
        <div class="topbar-title">{{ pageTitle }}</div>
        <div class="topbar-actions">
          <div class="theme-toggle" @click="toggleDark">
            <i :class="['pi', settingsStore.darkMode ? 'pi-moon' : 'pi-sun']" />
          </div>
        </div>
      </header>
      <main>
        <RouterView v-slot="{ Component }">
          <Transition name="page" mode="out-in">
            <component :is="Component" />
          </Transition>
        </RouterView>
      </main>
    </div>

    <Toast position="top-right" />
    <ConfirmDialog />
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  width: var(--app-sidebar-width);
  background: var(--app-surface);
  border-right: 1px solid var(--app-border);
  overflow: hidden;
  transition: width var(--transition);
}

.sidebar-header {
  display: flex;
  height: var(--app-header-height);
  border-bottom: 1px solid var(--app-border);
  align-items: center;
  justify-content: end;
  overflow: hidden;
  position: relative;
}

.sidebar-logo {
  display: flex;
  flex: 1;
  align-items: center;
  min-width: 0;
  gap: 0.67rem;
  overflow: hidden;
  position: absolute;
  left: 12px;
  transition:
    opacity var(--transition),
    visibility var(--transition);
}
.logo-mark {
  display: flex;
  flex-shrink: 0;
  width: 36px;
  height: 24px;
  background: #c3b1e1;
  color: #800080;
  border-radius: 3px;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}
.logo-text {
  font-size: 1.2rem;
  font-weight: 700;
  white-space: nowrap;
  overflow: hidden;
}

.sidebar-collapse-area {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  background: var(--app-surface);
  width: 56px;
  height: 32px;
  padding: 0 12px;
  z-index: 1;
  transition: width var(--transition);
}
.sidebar-collapse-area :deep(.p-button) {
  width: 100%;
  height: 100%;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  flex: 1;
  padding: 12px 12px;
  gap: 4px;
  overflow-y: auto;
}
.nav-item {
  display: flex;
  align-items: center;
  gap: 0.67rem;
  height: 32px;
  padding: 8px;
  border-radius: var(--app-button-radius);
  color: var(--app-text-muted);
  font-size: 1rem;
  text-decoration: none;
  overflow: hidden;
}
.nav-item span {
  transition:
    opacity var(--transition),
    visibility var(--transition);
}
.nav-item.active {
  background: var(--app-hover-active);
}
.nav-item:hover {
  background: var(--app-hover);
  color: var(--app-text);
}
.nav-item.active {
  color: var(--app-accent);
}
.nav-item .pi {
  font-size: 16px;
}

#app-shell.sidebar-collapsed .sidebar,
#app-shell.sidebar-collapsed .sidebar-collapse-area {
  width: 56px;
}
#app-shell.sidebar-collapsed .sidebar-logo,
#app-shell.sidebar-collapsed .nav-item span,
#app-shell.sidebar-collapsed .signout-btn {
  opacity: 0;
  visibility: hidden;
}

.main-area {
  display: flex;
  flex: 1;
  flex-direction: column;
  overflow: hidden;
}

.topbar {
  height: var(--app-header-height);
  border-bottom: 1px solid var(--app-border);
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 0 24px;
  flex-shrink: 0;
}
.topbar-title {
  font-size: 1.2rem;
  font-weight: 700;
  flex: 1;
}
.topbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.theme-toggle {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border: 1px solid var(--app-border);
  border-radius: var(--app-button-radius);
  color: var(--app-text-muted);
  transition:
    color var(--transition),
    border-color var(--transition);
  position: relative;
}
.theme-toggle:hover {
  color: var(--app-text);
  border-color: var(--app-text);
}

main {
  overflow-y: auto;
  padding: 12px 24px;
}
</style>
