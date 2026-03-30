import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    redirect: "/configuration",
  },
  {
    path: "/configuration",
    name: "configuration",
    component: () => import("./views/Configuration.vue"),
    meta: { title: "Configuration", icon: "pi-cog" },
  },
  {
    path: "/screensaver",
    name: "screensaver",
    component: () => import("./views/Screensaver.vue"),
    meta: { title: "Screensaver", icon: "pi-desktop" },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
