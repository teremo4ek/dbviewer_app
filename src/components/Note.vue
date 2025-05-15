<script setup>
import { ref } from 'vue';

import { useToast } from 'primevue/usetoast';
import Card from 'primevue/card';
import Checkbox from 'primevue/checkbox';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import ToggleSwitch from 'primevue/toggleswitch';

import { NOTE_TYPE } from '../const';

const { data } = defineProps(['data']);
const emit = defineEmits(['remove', 'update']);

const toast = useToast();

const note = ref({ ...data });
const isEditable = ref(false);

const edit = () => {
  if (!note.value.description) return;
  isEditable.value = false;
  emit('update', note.value);
};

const reset = () => {
  isEditable.value = false;
  note.value = { ...data };
};
</script>

<template>
  <div class="note elevate-2">
    <Card
      class="mb-2"
      :key="data.id"
      :pt="{
        body: {
          style: {
            padding: '10px',
          },
        },
      }"
    >
      <template #content>
        <div class="w-full flex align-items-center justify-content-between">
          <div class="flex align-items-center gap-2">
            <ToggleSwitch
              v-if="isEditable"
              v-model="note.status"
              trueValue="URGENT"
              falseValue="NORMAL"
              class="mr-2 flex-shrink-0"
              :pt="{
                slider: {
                  style: {
                    background: NOTE_TYPE[note.status].color,
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
                  :style="[
                    { color: NOTE_TYPE[note.status].color, fontSize: '0.7rem' },
                  ]"
                />
              </template>
            </ToggleSwitch>
            <i
              v-else
              :class="NOTE_TYPE[note.status].icon"
              :style="`color: ${NOTE_TYPE[note.status].color}`"
            />
            <InputText
              v-if="isEditable"
              v-model="note.description"
              id="description"
              class="w-full"
              :invalid="!note.description"
              autofocus
              autocomplete="off"
              :pt="{
                root: {
                  style: {
                    padding: '4px 8px',
                  },
                },
              }"
            />
            <p
              v-else
              :class="`m-0 text-500`"
            >
              {{ note.description }}
            </p>
          </div>
          <div v-if="isEditable">
            <Button
              icon="pi pi-check"
              @click="edit"
              variant="text"
              rounded
              severity="succ"
              size="small"
            />
            <Button
              icon="pi pi-times"
              class="ml-2"
              @click="reset"
              severity="danger"
              variant="text"
              rounded
              size="small"
            />
          </div>
          <div v-else>
            <Button
              icon="pi pi-pencil"
              @click="isEditable = true"
              variant="text"
              rounded
              severity="warn"
              size="small"
            />
            <Button
              icon="pi pi-trash"
              class="ml-2"
              @click="$emit('remove')"
              severity="danger"
              variant="text"
              rounded
              size="small"
            />
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>