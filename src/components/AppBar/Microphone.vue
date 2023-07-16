<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const microphone = ref(false);

const toggle = async () => {
  microphone.value = !microphone.value;

  if (microphone.value) {
    await invoke("audio_start");
  } else {
    await invoke("audio_stop");
  }
};
</script>

<template>
  <div>
    <Icon
      @click="toggle"
      :icon="microphone ? 'mdi:microphone' : 'mdi:microphone-off'"
    />
  </div>
</template>

<style scoped></style>
