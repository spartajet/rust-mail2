<template>
  <v-layout class="rounded rounded-md" style="overflow: hidden;">
    <v-app-bar height="3vh" ></v-app-bar>
    <v-navigation-drawer
        width="220"
        permanent style="overflow-y: hidden;"></v-navigation-drawer>

    <v-navigation-drawer
        width="300"
        permanent style="overflow-y: hidden;"
    ></v-navigation-drawer>
    <v-main class="d-flex align-center justify-center" style="height: 100vh; overflow: hidden;">
      <HelloWorld></HelloWorld>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
//
import HelloWorld from "@/components/HelloWorld.vue";
</script>
