<script setup>
import { ref } from 'vue';

import { invoke } from '@tauri-apps/api/core';

import InputText from 'primevue/inputtext';
import { useToast } from 'primevue/usetoast';

import Button from 'primevue/button';

const emit = defineEmits(['success']);

const toast = useToast();

const isLoading = ref(false);
const isLoadingChecker = ref(false);
const connectionPath = ref(import.meta.env.VITE_DATABASE_URL);

const connectDB = async () => {
  try {
    isLoading.value = true;
    await invoke('connect_to_db', {
      connectionstr: connectionPath.value,
    });
    emit('success');
  } catch (e) {
    toast.add({
      severity: 'error',
      summary: 'DB Error',
      detail: e,
      life: 3000,
    });
  } finally {
    isLoading.value = false;
  }
};

const checkForUpdates = async () => {
  try {
    isLoadingChecker.value = true;
    const res = await invoke('check_for_updates', {});
    if (!res) {
      toast.add({
        severity: 'info',
        summary: 'Update',
        detail: 'No updates available.',
        life: 3000,
      });
    }
  } catch (e) {
    toast.add({
      severity: 'error',
      summary: 'No updates available.',
      detail: e,
      life: 3000,
    });
  } finally {
    isLoadingChecker.value = false;
  }
};
</script>

<template>
  <div class="connect-screen flex justify-content-center">
    <div
      class="flex flex-column align-items-center w-30rem elevate-2 w-full gap-3"
    >
      <InputText
        v-model="connectionPath"
        id="connectionPath"
        class="w-full mb-2"
        :invalid="!connectionPath"
      />
      <Button
        label="Connect to DB"
        @click="connectDB"
        :loading="isLoading"
        :disabled="!connectionPath || isLoading"
      />
      <Button
        label="Check for updates"
        @click="checkForUpdates"
        :loading="isLoadingChecker"
        :disabled="!connectionPath || isLoadingChecker"
      />
    </div>
  </div>
</template>
