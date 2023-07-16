<script lang="ts" setup>
import { format } from "path";

const props = defineProps<{
  message: any;
  type: "user" | "system";
  index: number;
}>();

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
</script>

<template>
  <div
    :class="props.type == 'user' ? 'user' : 'system'"
    :style="`opacity: ${1 - (Math.min(props.index, 12) * 0.4) / 12}`"
  >
    <p>{{ props.message.content }}</p>
    <span> {{ formatTimestamp(props.message.started_at) }} </span>
  </div>
</template>

<style scoped>
div {
  width: fit-content;
  max-width: 80%;
  border-top-left-radius: 12px;
  border-top-right-radius: 12px;
  color: white;
  padding: 8px 12px 4px 12px;
  display: flex;
  flex-direction: column;
}

.system {
  margin: 6px auto 6px 6px;
  border-bottom-left-radius: 0px;
  border-bottom-right-radius: 12px;
  background-color: rgba(23, 23, 23, 0.9);
  align-content: flex-start;
}

.user {
  margin: 6px 6px 6px auto;
  border-bottom-left-radius: 12px;
  border-bottom-right-radius: 0px;
  background-color: rgba(65, 74, 169, 0.9);
  align-content: flex-end;
}

.user > span {
  text-align: right;
}

.user > p {
  text-align: right;
}

div > p {
  font-size: 10px;
  text-align: left;
  line-height: 1.5;
  margin: 0px;
}

div > span {
  font-size: 8px;
  text-align: left;
  margin: 0px;
}
</style>
