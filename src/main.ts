import Aura from "@primeuix/themes/aura";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import "primeicons/primeicons.css";
import PrimeVue from "primevue/config";
import ConfirmationService from "primevue/confirmationservice";
import KeyFilter from "primevue/keyfilter";
import ToastService from "primevue/toastservice";
import Tooltip from "primevue/tooltip";
import { createApp } from "vue";
import App from "./App.vue";
import "./assets/main.css";
import router from "./router";

// const appPreset = definePreset(Aura, {
//   semantic: {
//     colorScheme: {
//       light: {
//         semantic: {
//           primary: palette("{zink}"),
//           surface: palette("{zink}"),
//         },
//       },
//       dark: {
//         semantic: {
//           primary: palette("{indigo}"),
//           surface: palette("{slate}"),
//         },
//       },
//     },
//   },
// });

const app = createApp(App);

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

app.use(pinia);
app.use(router);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      prefix: "p",
      darkModeSelector: ".dark-mode",
      cssLayer: false,
    },
  },
});
app.use(ToastService);
app.use(ConfirmationService);
app.directive("tooltip", Tooltip);
app.directive("keyfilter", KeyFilter);

app.mount("#app");
