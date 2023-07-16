<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const blob = ref(null);

onMounted(async () => {
  await listen("screenshot", (event) => {
    console.log("BLOB");

    if ((event.payload as any).length && (event.payload as any)[0]) {
      blob.value = (event.payload as any)[0].base64_image;
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
  max-height: calc(100vh - 36px);
  height: calc(100vh - 36px);
  min-height: calc(100vh - 36px);
  padding: 0px;
  margin: 0px;
}

.fit-div {
  width: 100%;
  height: 100%;
}
</style>
