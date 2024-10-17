<template>
  <v-layout class="rounded rounded-md" style="overflow: hidden;">

    <v-navigation-drawer width="220" permanent style="overflow-y: hidden;"></v-navigation-drawer>
    <v-navigation-drawer width="300" permanent style="overflow-y: hidden;"></v-navigation-drawer>
    <v-app-bar height="40">
      <!-- <template v-slot:prepend>
        <v-app-bar-nav-icon></v-app-bar-nav-icon>

      </template> -->
      <v-sheet width="100%" height="100%" align="center" justify="center" @mousedown="title_bar_mouse_down_handle">
        <v-toolbar-title style="padding-top: 7px;">Rust Mail</v-toolbar-title>
      </v-sheet>

      <!-- <v-spacer></v-spacer>
      <v-app-bar-title>Rust Mail</v-app-bar-title> -->
      <template v-slot:append>
        <v-btn icon="mdi-plus-thick"></v-btn>
        <v-btn icon="mdi-cog"></v-btn>
        <v-btn class="window-control-btn" icon="mdi-window-minimize" @click="minimize_handle"></v-btn>
        <v-btn icon="mdi-window-maximize" @click="maximize_handle"></v-btn>
        <v-btn icon="mdi-window-close" @click="close_handle"></v-btn>
      </template>
    </v-app-bar>
    <v-main class="d-flex align-center justify-center" style="height: 100vh; overflow: hidden;">
      <HelloWorld></HelloWorld>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
//
import HelloWorld from "@/components/HelloWorld.vue";
import { getCurrentWindow } from '@tauri-apps/api/window';

const window = getCurrentWindow();

function title_bar_mouse_down_handle(e: MouseEvent) {
  if (e.buttons === 1) {
    e.preventDefault(); // Prevent dragging when clicking on title bar
    if (e.detail === 2) { // 检查是否为双击
      window.toggleMaximize(); // Maximize on double click
    } else {
      window.startDragging(); // Else start dragging
    }
  }
}

function minimize_handle() {
  window.minimize();
}
function maximize_handle() {
  window.toggleMaximize();
}
function close_handle() {
  window.close();
}
</script>
<style>
.window-control-btn {
  pointer-events: auto;
}
</style>
