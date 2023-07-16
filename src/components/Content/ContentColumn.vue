<script setup lang="ts">
import ContentTabsVue from "./ContentTabs.vue";
import ChatMessageVue from "./ChatMessage.vue";

import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const messages = ref<Array<any>>([]);

onMounted(async () => {
  await listen("content", (event: any) => {
    console.log("event", event);
    for (const item of event.payload) {
      const ids = messages.value.map((message: any) => message.id);

      if (!ids.includes(item.id)) {
        // Find the right index to insert while respecting ascending createdAt order
        let index = 0;

        for (const id of ids) {
          if (item.id > id) index++;
          else break;
        }

        messages.value.splice(index, 0, item);
      }
    }

    messages.value = messages.value.sort(
      (a: any, b: any) => a.timestamp - b.timestamp
    );

    console.log("Messages", messages.value);
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
        :index="messages.length - index"
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
  max-height: calc(100vh - 72px);
  overflow-y: scroll;
  flex-direction: column;
  display: flex;
}
</style>
