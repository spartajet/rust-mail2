<template>
  <v-layout style="overflow: hidden">
    <AccountNav></AccountNav>

    <v-navigation-drawer width="300" permanent style="overflow-y: hidden">
      <MailList></MailList>
    </v-navigation-drawer>

    <v-main
      class="d-flex align-center justify-center"
      style="height: 100vh; overflow: hidden"
    >
      <!-- <HelloWorld></HelloWorld> -->
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
//
// import HelloWorld from "@/components/HelloWorld.vue";
// import { getCurrentWindow } from '@tauri-apps/api/window';
import MailList from "@/components/MailList.vue";
import AccountNav from "@/components/AccountNav.vue";

// const window = getCurrentWindow();

// function title_bar_mouse_down_handle(e: MouseEvent) {
//   if (e.buttons === 1) {
//     e.preventDefault(); // Prevent dragging when clicking on title bar
//     if (e.detail === 2) { // 检查是否为双击
//       window.toggleMaximize(); // Maximize on double click
//     } else {
//       window.startDragging(); // Else start dragging
//     }
//   }
// }

// function minimize_handle() {
//   window.minimize();
// }
// function maximize_handle() {
//   window.toggleMaximize();
// }
// function close_handle() {
//   window.close();
// }
</script>
<style>
/* .window-control-btn {
  pointer-events: auto;
} */
</style>
