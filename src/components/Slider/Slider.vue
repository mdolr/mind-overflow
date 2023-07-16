<script lang="ts" setup>
import { ref, onMounted, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const slider = ref(50);
const oldest = ref(0);
const newest = ref(100);
const date = ref("Loading...");

const formatTimestamp = (timestamp: any) => {
  timestamp = new Date(timestamp);

  // Returns timestamp as dd/mm/yyyy hh:mm:ss
  // pad with 0 if necessary to keep 2 digits consistently
  const pad = (n: any) => (n < 10 ? "0" + n : n);

  return `${pad(timestamp.getDate())}/${pad(
    timestamp.getMonth() + 1
  )}/${timestamp.getFullYear()} ${pad(timestamp.getHours())}:${pad(
    timestamp.getMinutes()
  )}:${pad(timestamp.getSeconds())}`;
};

onMounted(async () => {
  await listen("screenshot", async (event: any) => {
    if (event.payload.origin != "slider") await invoke("db_get_slider");
  });

  await listen("content", async (event: any) => {
    if (event.payload.origin != "slider") await invoke("db_get_slider");
  });

  await listen("slider", async (event: any) => {
    oldest.value = event.payload[0].lowest_created_at;
    newest.value = event.payload[0].highest_created_at;

    date.value = formatTimestamp(event.payload[0].highest_created_at);
    console.log("slider event received");
  });

  await invoke("db_get_slider");
});

watch(slider, (newVal) => {
  const timestamp =
    ((newest.value - oldest.value) * newVal) / 100 + oldest.value;
  date.value = formatTimestamp(timestamp);
});

const onSliderChange = () => {
  const timestamp =
    ((newest.value - oldest.value) * slider.value) / 100 + oldest.value;
  invoke("db_get_content", { before: timestamp, origin: "slider" });
  invoke("db_get_screenshot", { before: timestamp, origin: "slider" });
};
</script>

<template>
  <div class="slider-parent">
    <div>
      <div class="slider-date">{{ date }}</div>
      <input
        type="range"
        min="0"
        max="100"
        step="0.001"
        v-model="slider"
        class="slider"
        @change="onSliderChange"
        :disabled="oldest == 0"
      />
    </div>
  </div>
</template>

<style scoped>
.slider-parent {
  width: 100vw;
  height: 100%;
  min-height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  align-content: center;
  justify-items: center;
}

.slider-date {
  width: 100vw;
  text-align: center;
  margin-bottom: 6px;
}

.slider {
  width: 90%;
  -webkit-appearance: none;
  height: 10px;
  border-radius: 5px;
  background: white;
  cursor: grab;
}
</style>
