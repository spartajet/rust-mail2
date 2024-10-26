<template>
  <v-container style="padding: 0">
    <v-row>
      <v-col style="padding-top: o; padding-bottom: 0">
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
      </v-col>
    </v-row>
    <v-row>
      <v-col style="padding-top: 0">
        <v-infinite-scroll style="padding: 0">
          <v-card
            v-for="i in 50"
            v-bind:key="i"
            style="height: 65px; margin: 3px; border: 1px black"
          >
            <v-container style="padding: 3px">
              <v-row no-gutters>
                <v-col class="font-weight-bold" style="font-size: 10px"
                  >Dr.Guo guoxiaozhong1990@outloo.com</v-col
                >
                <v-col style="font-size: 10px; max-width: 80px">
                  2024-01-01 01:22
                </v-col>
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
          </v-card>
        </v-infinite-scroll>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-card color="surface-light" width="100%" height="30px"> </v-card>
      </v-col>
    </v-row>
  </v-container>

  <!--  <v-sheet class="list-sheet" width="100%">-->

  <!--  </v-sheet>-->
</template>
<script setup lang="ts">
import { ref } from "vue";

const loaded = ref(false);
const loading = ref(false);

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
</style>
