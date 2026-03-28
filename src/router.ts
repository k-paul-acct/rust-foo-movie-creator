import { createRouter, createWebHistory } from "vue-router";
import Configuration from "./views/Configuration.vue";
import Screensaver from "./views/Screensaver.vue";

const routes = [
  {
    path: "/",
    redirect: "/configuration",
  },
  {
    path: "/configuration",
    name: "configuration",
    component: Configuration,
    meta: { title: "Configuration", icon: "pi-cog" },
  },
  {
    path: "/screensaver",
    name: "screensaver",
    component: Screensaver,
    meta: { title: "Screensaver", icon: "pi-desktop" },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
