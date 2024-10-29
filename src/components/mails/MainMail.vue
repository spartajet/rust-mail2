<template>
  <v-navigation-drawer width="300" permanent style="overflow-y: hidden">
    <MailList></MailList>
  </v-navigation-drawer>

  <v-main
    class="d-flex align-center justify-center"
    style="height: 100vh; overflow: hidden"
  >
    <MailDetailPage></MailDetailPage>
  </v-main>
</template>
<script setup lang="ts">
import MailList from "@/components/mails/MailList.vue";
import MailDetailPage from "@/components/mails/MailDetailPage.vue";
</script>
