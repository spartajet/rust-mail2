<template>
  <v-layout>
    <v-app-bar
      id="find-bar"
      style="padding-top: 0; padding-bottom: 0; height: 40px"
    >
      <v-card
        class="mx-auto"
        color="surface-light"
        width="100%"
        height="40px"
        style="border-radius: 2px"
      >
        <v-card-text style="padding: 2px">
          <v-text-field
            :loading="loading"
            append-inner-icon="mdi-magnify"
            density="compact"
            label="Search Anything"
            variant="solo"
            hide-details
            single-line
            @click:append-inner="onClick"
            style="height: 28px"
          ></v-text-field>
        </v-card-text>
      </v-card>
    </v-app-bar>
    <v-main style="padding-top: 41px; padding-bottom: 20px">
      <v-infinite-scroll
        :height="'calc(100vh - 60px)'"
        :items="items"
        style="padding: 0"
        side="end"
      >
        <template v-for="item in items" :key="item">
          <div
            style="height: 65px; margin: 3px; border: 1px black; width: 300px"
          >
            <MailCard></MailCard>
          </div>
        </template>
      </v-infinite-scroll>
    </v-main>
    <v-app-bar
      id="bottom-bar"
      color="grey-lighten-2"
      style="height: 20px; padding-top: 0;padding-left: 20px;"
      location="bottom"
      flat
    >
      <div class="text-subtitle-2">共有 {{ items.length }} 封邮件</div>
    </v-app-bar>
  </v-layout>

  <!--  <v-sheet class="list-sheet" width="100%">-->

  <!--  </v-sheet>-->
</template>
<script setup lang="ts">
import { ref } from "vue";
import MailCard from "@/components/MailCard.vue";

const loaded = ref(false);
const loading = ref(false);
const items = ref(Array.from({ length: 100 }, (k, v) => v + 1));

function onClick() {
  loading.value = true;

  setTimeout(() => {
    loading.value = false;
    loaded.value = true;
  }, 2000);
}
</script>
<style>
.list-sheet {
  height: calc(100% - 70px);
}

.v-btn {
  height: 12px !important;
  width: 12px !important;
  margin: 4px;
}

.v-btn--size-x-small {
  --v-btn-height: 15px;
}

.v-btn__content {
  height: 12px !important;
  width: 12px !important;
}

.v-icon {
  height: 12px !important;
  width: 12px !important;
}

.v-btn__underlay {
  height: 15px;
  width: 15px;
}

.v-icon--size-default {
  font-size: 16px !important;
}

.v-badge__badge {
  font-size: 10px !important;
  padding: 2px 5px !important;
}

.v-badge {
  height: 13px !important;
}

.v-badge__wrapper {
  margin: 0px !important;
}
.v-infinite-scroll__side {
  padding: 0px !important;
}
#find-bar .v-toolbar__content {
  height: 40px !important;
}
#bottom-bar .v-toolbar__content {
  height: 20px !important;
}
</style>
