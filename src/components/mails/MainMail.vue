<template>
  <v-navigation-drawer width="300" permanent style="overflow-y: hidden">
    <MailList></MailList>
  </v-navigation-drawer>
  <v-app-bar style="height: 120px; padding: 0">
    <v-container>
      <v-row>
        <v-col cols="12">
          <h2 class="text-h6 font-weight-bold ml-8">请准时参加会议</h2>
        </v-col>
      </v-row>
      <v-row>
        <v-col cols="12">
          <div class="d-flex align-center title-row">
            <v-icon>mdi-account</v-icon>
            <span class="ml-2 text-body-2">发件人: leader@gmail.com</span>
          </div>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <div class="d-flex align-center title-row">
            <v-icon>mdi-account-multiple</v-icon>
            <span class="ml-2 text-body-2">收件人: dfad@gmail.com</span>
          </div>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <div class="d-flex align-center title-row">
            <v-icon>mdi-account-multiple-outline</v-icon>
            <span class="ml-2 text-body-2">抄送: dfaddd@gmail.com</span>
          </div>
        </v-col>
      </v-row>
    </v-container>
  </v-app-bar>

  <v-main
    class="d-flex"
    style="height: calc(100vh); padding-top: 120px; margin: 10px"
  >
    <div v-html="sanitizedMailContent"></div>
  </v-main>
  <v-app-bar location="bottom" style="height: 60px"></v-app-bar>
</template>
<script setup lang="ts">
import MailList from "./MailList.vue";
import DOMPurify from "dompurify";
import { ref, computed } from "vue";

// 假设这是从props或store获取的邮件HTML内容
const mailContent = ref("<p>这是邮件内容...</p>");

// 计算属性用于净化HTML内容
const sanitizedMailContent = computed(() => {
  return DOMPurify.sanitize(mailContent.value);
});
</script>
<style>
.v-app-bar {
  padding: 0 !important;
}
.v-toolbar__content {
  padding: 0 !important;
  height: 100% !important;
}
.v-container {
  padding: 8px !important;
}
.v-row {
  margin: 0 !important;
}
.v-col {
  padding: 0 !important;
}
.title-row {
  height: 30px !important;
}
</style>
