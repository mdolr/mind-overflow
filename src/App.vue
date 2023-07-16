<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

import TogglesVue from "./components/AppBar/Toggles.vue";
import ContentTabsVue from "./components/Content/ContentTabs.vue";
import SystemMessageVue from "./components/Content/SystemMessage.vue";
import UserMessageVue from "./components/Content/UserMessage.vue";
import ContentColumnVue from "./components/Content/ContentColumn.vue";
import ScreenVue from "./components/Screenshot/Screen.vue";
import SliderVue from "./components/Slider/Slider.vue";
</script>

<template>
  <div class="container">
    <div class="topbar"><TogglesVue /></div>
    <div class="columns">
      <div class="column column1">
        <ScreenVue />
      </div>
      <ContentColumnVue style="width: calc(100% - 66%); max-height: 10vh" />
    </div>
    <div class="slider-container"><SliderVue /></div>
  </div>
</template>

<style>
html,
body {
  margin: 0;
  padding: 0;
}

.slider-container {
  width: 100vw;
  height: 100px;
  padding: 0px 0px;
  min-height: 100px;
  background-color: #111;
  border-top: 1px solid rgba(229, 231, 235, 0.25);
  display: flex;
}

.columns {
  display: flex;
  width: 100vw;
  height: calc(100vh - 136px);
  max-height: calc(100vh - 136px);
  min-height: calc(100vh - 136px);
  flex-direction: row;
}

.columns > .column {
  min-height: 100%;
}

.column1 {
  width: 66%;
}

.topbar {
  background: #111;
  border-bottom: 1px solid rgba(229, 231, 235, 0.25);
  height: 36px;
  width: 100vw;
  display: flex;
  flex-direction: row;
  align-items: center;
}

.container {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  align-items: start;
  justify-content: start;
  padding: 0;
  margin: 0;
  font-family: monospace;
  background-color: #111;
  background: linear-gradient(
      0deg,
      rgba(229, 231, 235, 0.05) 1px,
      transparent 1px
    ),
    linear-gradient(90deg, rgba(229, 231, 235, 0.05) 1px, transparent 1px);
  background-size: 20px 20px;
  background-position: 0 0, 0 0;
}
</style>
