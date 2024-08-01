<template>
  <q-layout view="lHh LpR lFf">
    <q-drawer
      show-if-above
      v-model="leftDrawerOpen"
      side="left"
      behavior="desktop"
      bordered
    >
      <div class="column" style="height: 100%">
        <div style="height: 100px">1 of 2</div>
        <div class="col">1 of 2</div>
        <div style="height: 100px">1 of 2</div>
      </div>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { ref } from "vue";

export default {
  setup() {
    const leftDrawerOpen = ref(true);

    return {
      leftDrawerOpen,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value;
      },
    };
  },
};
</script>

<style lang="sass" scoped>
.column > div
  padding: 10px 15px
  background: rgba(86, 61, 124, .15)
  border: 1px solid rgba(86, 61, 124, .2)
.column + .column
  margin-top: 1rem
</style>
