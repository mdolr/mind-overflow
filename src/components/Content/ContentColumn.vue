<script setup lang="ts">
import ContentTabsVue from "./ContentTabs.vue";
import ChatMessageVue from "./ChatMessage.vue";

import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const messages = ref<Array<any>>([]);

onMounted(async () => {
  await listen("content", (event: any) => {
    // console.log("event", event);
    messages.value = event.payload.rows
      .sort((a: any, b: any) => a.id - b.id)
      .reverse();

    // console.log("Messages", messages.value);
  });

  await invoke("db_get_content");
});
</script>

<template>
  <div class="column column2">
    <ContentTabsVue />

    <div class="scrollable">
      <ChatMessageVue
        v-for="(message, index) in messages"
        :key="message.id"
        :message="message"
        :type="'user'"
        :index="index"
      />
    </div>
  </div>
</template>

<style scoped>
div {
  border-color: rgba(229, 231, 235, 0.25);
  border-style: solid;
  border-width: 0 0px 0px 1px;
}

.scrollable {
  max-height: calc(100vh - 172px);
  overflow-y: scroll;
  flex-direction: column-reverse;
  display: flex;
}
</style>
