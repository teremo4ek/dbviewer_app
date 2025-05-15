<script setup>
import { invoke } from '@tauri-apps/api/core';

import { ref, onMounted } from 'vue';

import DynamicDialog from 'primevue/dynamicdialog';
import Notes from './Notes.vue';
import ProgressSpinner from 'primevue/progressspinner';
import Badge from 'primevue/badge';

const { version } = defineProps(['version']);
const emit = defineEmits(['trouble-db']);
</script>

<template>
  <div class="flex align-items-center flex-column h-full w-full">
    <div class="flex align-items-center my-4">
      <h1 class="mr-2 m-0 text-500">Demo Notes</h1>
      <Badge
        size="middle"
        severity="secondary"
        >v{{ version }}</Badge
      >
    </div>
    <div class="w-30rem">
      <Notes @trouble-db="emit('trouble-db')" />
    </div>
  </div>
</template>
