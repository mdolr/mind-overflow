<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const screen = ref(false);

const toggle = async () => {
  screen.value = !screen.value;

  if (screen.value) {
    await invoke("screen_start");
  } else {
    await invoke("screen_stop");
  }
};
</script>

<template>
  <div>
    <Icon
      @click="toggle"
      :icon="
        screen
          ? 'mdi:projector-screen-variant'
          : 'mdi:projector-screen-variant-off'
      "
    />
  </div>
</template>

<style scoped></style>
