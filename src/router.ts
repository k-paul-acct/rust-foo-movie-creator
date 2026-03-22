import { createRouter, createWebHashHistory } from "vue-router";
import Effects from "./views/Effects.vue";
import Images from "./views/Images.vue";
import Output from "./views/Output.vue";
import Screensaver from "./views/Screensaver.vue";

const routes = [
  {
    path: "/",
    redirect: "/output",
  },
  {
    path: "/output",
    name: "output",
    component: Output,
    meta: { title: "Output", icon: "pi-cog" },
  },
  {
    path: "/images",
    name: "images",
    component: Images,
    meta: { title: "Images", icon: "pi-images" },
  },
  {
    path: "/screensaver",
    name: "screensaver",
    component: Screensaver,
    meta: { title: "Screensaver", icon: "pi-desktop" },
  },
  {
    path: "/effects",
    name: "effects",
    component: Effects,
    meta: { title: "Effects", icon: "pi-star" },
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
