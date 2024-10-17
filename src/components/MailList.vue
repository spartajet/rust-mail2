<template>


  <v-card class="mx-auto" color="surface-light" width="100%" height="6%" min-height="60px">
    <v-card-text style="padding: 10px;">
      <v-text-field :loading="loading" append-inner-icon="mdi-magnify" density="compact" label="Search Anything"
        variant="solo" hide-details single-line @click:append-inner="onClick"></v-text-field>
    </v-card-text>
  </v-card>


  <v-sheet width="100%" height="90%">

  </v-sheet>
  <v-card color="surface-light" width="100%" height="4%" min-height="40px">

  </v-card>




</template>
<script setup lang="ts">
import { ref } from 'vue'
const loaded = ref(false)
const loading = ref(false)


function onClick() {
  loading.value = true

  setTimeout(() => {
    loading.value = false
    loaded.value = true
  }, 2000)
}
</script>
