<script setup lang="ts">
import MicrophoneVue from "./Microphone.vue";
import ScreenVue from "./Screen.vue";
</script>

<template>
  <div class="toggles">
    <ScreenVue />
    <MicrophoneVue />
  </div>
</template>

<style scoped>
div {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  margin-left: auto;
  margin-right: 4px;
}

.toggles > div {
  margin: 0 2px;

  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(47, 47, 47);
  border: solid 1px rgba(229, 231, 235, 0.25);
  transition: 0.15s;
  -webkit-touch-callout: none; /* iOS Safari */
  -webkit-user-select: none; /* Safari */
  -khtml-user-select: none; /* Konqueror HTML */
  -moz-user-select: none; /* Old versions of Firefox */
  -ms-user-select: none; /* Internet Explorer/Edge */
  user-select: none; /* Non-prefixed version, currently
                                  supported by Chrome, Edge, Opera and Firefox */
  cursor: pointer;
}

.toggles > div:hover {
  background-color: rgba(47, 47, 47, 0.5);
}
</style>
