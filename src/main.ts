import { createApp } from 'vue';
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import ToastService from 'primevue/toastservice';

import Entry from './Entry.vue';

import 'primeflex/primeflex.css';
import 'primeicons/primeicons.css';

createApp(Entry)
  .use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        darkModeSelector: '.p-dark',
        cssLayer: false,
      }
    },
  })
  .use(ToastService)
  .mount('#app');
