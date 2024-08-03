<template>
  <q-layout view="hHh lpR lFf">
    <q-drawer
      v-model="leftDrawerOpen"
      side="left"
      behavior="desktop"
      bordered
      :width="300"
    >
      <q-pull-to-refresh @refresh="refresh">
        <q-item v-for="n in 20" :key="n">
          <q-item-section avatar>
            <q-skeleton type="QAvatar" />
          </q-item-section>

          <q-item-section>
            <q-item-label>
              <q-skeleton type="text" />
            </q-item-label>
            <q-item-label caption>
              <q-skeleton type="text" width="95%" />
            </q-item-label>
          </q-item-section>
        </q-item>
      </q-pull-to-refresh>
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
