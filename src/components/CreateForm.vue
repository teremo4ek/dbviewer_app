<script setup>
import { invoke } from '@tauri-apps/api/core';

import { ref, useTemplateRef, onMounted } from 'vue';

import { useToast } from 'primevue/usetoast';

import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Checkbox from 'primevue/checkbox';
import ToggleSwitch from 'primevue/toggleswitch';
import { NOTE_TYPE } from '../const';

const emit = defineEmits(['created', 'cancel', 'trouble-db']);

const toast = useToast();

const input = useTemplateRef('my-input');

const defaultStateForm = {
  status: 'NORMAL',
  description: '',
};

const form = ref({ ...defaultStateForm });

const create = async () => {
  try {
    if (!form.value.description.trim()) return;
    await invoke('add_note', form.value);
    toast.add({
      severity: 'success',
      summary: 'Create',
      detail: `${form.value.description} created`,
      life: 3000,
    });
    emit('created');
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

const reset = () => {
  form.value = { ...defaultStateForm };
  creating.value = false;
};

onMounted(() => {
  input.value.$el.focus();
});
</script>

<template>
  <div class="flex mb-2 align-items-center">
    <ToggleSwitch
      v-model="form.status"
      falseValue="NORMAL"
      trueValue="URGENT"
      class="mr-2 flex-shrink-0"
      :pt="{
        slider: {
          style: {
            background: NOTE_TYPE[form.status].color,
          },
        },
      }"
    >
      <template #handle="{ checked }">
        <i
          :class="[
            '!text-xs pi',
            { 'pi-sun': !checked, 'pi-megaphone': checked },
          ]"
          :style="[{ color: NOTE_TYPE[form.status].color, fontSize: '0.7rem' }]"
        />
      </template>
    </ToggleSwitch>
    <InputText
      ref="my-input"
      v-model="form.description"
      id="description"
      class="w-full"
      autocomplete="off"
      :invalid="!form.description.trim()"
      :pt="{
        root: {
          style: {
            padding: '5px 8px',
          },
        },
      }"
    />
    <Button
      icon="pi pi-check"
      class="ml-2 flex-shrink-0"
      @click="create"
    />
    <Button
      icon="pi pi-times"
      class="ml-2 flex-shrink-0"
      @click="emit('cancel')"
      severity="danger"
    />
  </div>
</template>