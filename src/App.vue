<template>
  <v-layout class="rounded rounded-md">
    <v-system-bar color="grey-darken-3"></v-system-bar>
    <v-navigation-drawer
        color="grey-darken-2"
        width="150"
        permanent
    ></v-navigation-drawer>

    <v-navigation-drawer
        color="grey-darken-1"
        width="150"
        permanent
    ></v-navigation-drawer>
    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
      <HelloWorld></HelloWorld>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
//
import HelloWorld from "@/components/HelloWorld.vue";
</script>
