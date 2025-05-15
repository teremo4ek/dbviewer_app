<script setup>
import { invoke } from '@tauri-apps/api/core';

import { ref, computed, onMounted } from 'vue';

import { useToast } from 'primevue/usetoast';

import Card from 'primevue/card';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Checkbox from 'primevue/checkbox';
import Dialog from 'primevue/dialog';
import Badge from 'primevue/badge';
import ScrollPanel from 'primevue/scrollpanel';
import ToggleSwitch from 'primevue/toggleswitch';

import CreateForm from './CreateForm.vue';
import Search from './Search.vue';
import Note from './Note.vue';

const toast = useToast();

const emit = defineEmits(['trouble-db']);

const notes = ref([]);
const creating = ref(false);

const getData = async (query = '') => {
  try {
    const res = await invoke('search_note', { query });
    notes.value = res.sort(({ status }) => (status === 'URGENT' ? -1 : 1));
  } catch (e) {
    toast.add({
      severity: 'error',
      summary: 'DB Error',
      detail: e,
      life: 3000,
    });
    emit('trouble-db');
  }
};

onMounted(() => {
  getData();
});

const updateNote = async (note) => {
  try {
    await invoke('update_note', { note });
    toast.add({
      severity: 'success',
      summary: 'Update',
      detail: `${note.description} updated`,
      life: 3000,
    });
    getData();
  } catch (e) {
    toast.add({
      severity: 'error',
      summary: 'DB Error',
      detail: e,
      life: 3000,
    });
    emit('trouble-db');
  }
};

const removeNote = async (note) => {
  try {
    await invoke('delete_note', { id: note.id });
    toast.add({
      severity: 'success',
      summary: 'Remove',
      detail: `${note.description} removed`,
      life: 3000,
    });
    getData();
  } catch (e) {
    toast.add({
      severity: 'error',
      summary: 'DB Error',
      detail: e,
      life: 3000,
    });
    emit('trouble-db');
  }
};

const NORMAL = computed(
  () => notes.value.filter(({ status }) => status === 'NORMAL').length,
);
const URGENT = computed(
  () => notes.value.filter(({ status }) => status === 'URGENT').length,
);
const TOTAL = computed(() => notes.value.length);
</script>


<template>
  <div class="flex justify-content-center mb-2">
    <Badge
      size="large"
      class="mr-2 w-full"
      severity="info"
      ><i class="pi pi-pen-to-square mr-2" /> Total: {{ TOTAL }}</Badge
    >
    <Badge
      size="large"
      class="mr-2 w-full"
      severity="warn"
      ><i class="pi pi-sun mr-2" /> Normal: {{ NORMAL }}</Badge
    >
    <Badge
      size="large"
      class="w-full"
      severity="danger"
      ><i class="pi pi-megaphone mr-2" /> Urgent: {{ URGENT }}</Badge
    >
  </div>
  <div class="flex justify-content-between mb-2">
    <search
      :disabled="creating"
      @change="getData"
    />
    <Button
      icon="pi pi-plus"
      class="ml-2 flex-shrink-0"
      @click="creating = true"
      :disabled="creating"
    />
  </div>

  <create-form
    v-if="creating"
    @trouble-db="emit('trouble-db')"
    @cancel="creating = false"
    @created="
      () => {
        getData();
        creating = false;
      }
    "
  />

  <div
    v-if="!notes.length"
    class="flex align-items-center justify-content-center shadow-2 h-5rem border-round text-300"
  >
    <span class="font-semibold m-0">Empty notes</span>
  </div>

  <div
    v-else
    style="width: 100%"
  >
    <Note
      v-for="item in notes"
      :key="item.id"
      :data="item"
      @update="updateNote"
      @remove="removeNote(item)"
    />
  </div>
</template>

<style scoped></style>