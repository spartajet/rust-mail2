<template>
  <v-container style="padding: 3px">
    <v-row no-gutters>
      <v-col class="font-weight-bold" style="font-size: 10px"
        >Dr.Guo guoxiaozhong1990@outloo.com</v-col
      >
      <v-col style="font-size: 10px; max-width: 80px"> 2024-01-01 01:22 </v-col>
    </v-row>
    <v-row
      class="text-truncate font-weight-bold"
      no-gutters
      style="font-size: 12px; height: 20px; display: block"
    >
      请准时参加会议，谢谢，另外准备好会议材料
    </v-row>
    <v-row no-gutters>
      <v-col>
        <v-badge content="会议" inline></v-badge>
        <v-badge content="领导" inline></v-badge>
        <v-badge content="非常重要" inline></v-badge>
      </v-col>
      <v-col style="max-width: 60px">
        <v-btn icon size="12" variant="text">
          <v-icon>mdi-star</v-icon>
        </v-btn>
        <v-btn icon variant="text">
          <v-icon>mdi-flag</v-icon>
        </v-btn>
        <v-btn icon variant="text">
          <v-icon>mdi-pen</v-icon>
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>
