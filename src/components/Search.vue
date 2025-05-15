<script setup>
import { ref, watch } from 'vue';

import InputText from 'primevue/inputtext';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';

function debounce(func, timeout = 300){
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => { func.apply(this, args); }, timeout);
  };
}

const emit = defineEmits(['change']);
const { disabled } = defineProps(['disabled']);

const value = ref('');

const DBChange = debounce((value) => emit('change', value));

watch(value, DBChange);

watch(() => disabled, () => value.value = '');

</script>

<template>
  <IconField class="w-full">
    <InputIcon class="pi pi-search" />
    <InputText
      v-model="value"
      placeholder="Search"
      class="w-full"
      :disabled="disabled"
      type="search"
    />
  </IconField>
</template>