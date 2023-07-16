<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const blob = ref(null);

onMounted(async () => {
  await listen("screenshot", (event) => {
    console.log("BLOB", event);

    if ((event.payload as any).rows.length && (event.payload as any).rows[0]) {
      blob.value = (event.payload as any).rows[0].base64_image;
    }
  });

  await invoke("db_get_screenshot");
});
</script>

<template>
  <div>
    <img :src="`data:image/png;base64,${blob}`" class="fit-div" />
  </div>
</template>

<style scoped>
div {
  max-height: calc(100vh - 136px);
  height: calc(100vh - 136px);
  min-height: calc(100vh - 136px);
  padding: 0px;
  margin: 0px;
}

.fit-div {
  width: 100%;
  height: 100%;
  border-radius: 0px !important;
}
</style>
