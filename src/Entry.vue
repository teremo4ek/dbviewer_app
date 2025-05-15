<script setup>
import { ref, onMounted } from 'vue';

import { invoke } from '@tauri-apps/api/core';

import Badge from 'primevue/badge';
import Toast from 'primevue/toast';

import { useToast } from 'primevue/usetoast';

import ConnectDB from './components/ConnectDB.vue';
import App from './components/App.vue';

const toast = useToast();

const connected = ref(false);
const version = ref('');

const init = async () => {
  try {
    version.value = await invoke('get_app_version', {});
  } catch (e) {
    toast.add({
      severity: 'error',
      summary: 'DB Error',
      detail: e,
      life: 3000,
    });
  }
};

onMounted(() => {
  init();
});
</script>

<template>
  <Toast position="bottom-left" />
  <div
    v-if="!connected"
    class="mt-6 flex flex-column align-items-center"
  >
    <ConnectDB @success="connected = true" />
    <Badge
      class="mt-4"
      size="xlarge"
      severity="secondary"
      >v{{ version }}</Badge
    >
  </div>
  <App
    v-else
    :version="version"
    @trouble-db="connected = false"
  />
</template>

<style>
:root {
  font-family: Roboto, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
}
</style>
