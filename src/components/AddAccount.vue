<template>
  <q-dialog v-model="displayDialog">
    <div
      class="q-pa-md full-width row inline wrap justify-start items-start content-start"
      style="width: 600px"
    >
      <q-card
        class="my-card q-ma-sm"
        v-for="item in accountItems"
        :key="item.id"
        style="width: 100px; height: 140px"
      >
        <q-img
          :src="`src/assets/mail-logos/${item.icon}.png`"
          fit="contain"
          class="q-pa-md"
          style="height: 90px"
        >
          <!-- <div class="absolute-bottom text-subtitle2 text-center">
            {{ item.name }}
          </div> -->
        </q-img>
        <q-separator />

        <q-card-actions
          align="center"
          class="q-pa-none q-ma-none"
          style="border: 0px"
          width="100%"
        >
          <q-btn
            square
            flat
            class="q-ma-none q-mt-xs q-pa-none"
            style="width: 100%; height: 100%"
            >{{ item.name }}</q-btn
          >
        </q-card-actions>
      </q-card>
    </div>
  </q-dialog>
</template>
<script>
import { ref } from "vue";

export default {
  setup() {
    const displayDialog = ref(false);
    const accountItems = ref([
      {
        id: "gmail",
        name: "Gmail",
        icon: "gmail",
      },
      {
        id: "outlook",
        name: "Outlook",
        icon: "outlook",
      },
      {
        id: "yahoo",
        name: "Yahoo",
        icon: "yahoo",
      },
      {
        id: "icloud",
        name: "iCloud",
        icon: "icloud",
      },
      {
        id: "yi",
        name: "网易",
        icon: "yi",
      },
      {
        id: "exchange",
        name: "Exchange",
        icon: "exchange",
      },
    ]);

    return {
      accountItems,
      displayDialog,
    };
  },
};
</script>
