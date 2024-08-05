<template>
  <q-layout view="hHh lpR lFf">
    <q-drawer
      v-model="leftDrawerOpen"
      side="left"
      behavior="desktop"
      bordered
      :width="300"
    >
      <q-input
        outlined
        bottom-slots
        label="search anything"
        stack-label
        dense
        class="q-ma-xs"
      >
        <template v-slot:prepend>
          <q-icon name="place" />
        </template>

        <template v-slot:control>
          <div class="self-left full-width no-outline" tabindex="0">
            {{ text }}
          </div>
        </template>

        <template v-slot:append>
          <q-icon name="close" class="cursor-pointer" />
          <q-icon name="search" />
        </template>
      </q-input>
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
      <MailDetailPage />
    </q-page-container>
  </q-layout>
</template>

<script>
import { ref } from "vue";
import MailDetailPage from "src/pages/MailDetailPage.vue";

export default {
  components: {
    MailDetailPage,
  },
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
