<template>
  <div class="mail-detail" style="height: 100vh; padding: 20px;">
    <!-- 邮件标题区域 -->
    <div class="mail-header" style="margin-bottom: 20px;">
      <h2 class="text-h5">{{ mail.title }}</h2>
      <div class="d-flex align-center" style="margin-top: 10px;">
        <v-icon>mdi-account</v-icon>
        <span class="ml-2">发件人: {{ mail.from }}</span>
      </div>
    </div>

    <!-- 收件人和抄送区域 -->
    <div class="mail-recipients" style="margin-bottom: 20px;">
      <div class="d-flex align-center mb-2">
        <v-icon>mdi-account-multiple</v-icon>
        <span class="ml-2">收件人: {{ mail.to.join('; ') }}</span>
      </div>
      <div class="d-flex align-center" v-if="mail.cc.length">
        <v-icon>mdi-account-multiple-outline</v-icon>
        <span class="ml-2">抄送: {{ mail.cc.join('; ') }}</span>
      </div>
    </div>

    <v-divider></v-divider>

    <!-- 邮件正文区域 -->
    <div class="mail-content" style="margin-top: 20px;">
      <div v-html="mail.content"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const mail = ref({
  title: '关于项目进度会议的通知',
  from: 'manager@company.com',
  to: ['employee1@company.com', 'employee2@company.com'],
  cc: ['supervisor@company.com'],
  content: `
    <p>各位同事：</p>
    <p>定于明天下午2点在会议室召开项目进度讨论会议，请准时参加。</p>
    <p>请各位准备好相关材料。</p>
    <br>
    <p>谢谢！</p>
  `
});
</script>
