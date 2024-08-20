<template>
  <div class="q-pa-sm column" style="height: 100vh">
    <div style="height: 100px">
      <q-skeleton type="text">.col</q-skeleton>
    </div>
    <q-separator />
    <div class="col">.col-5</div>
    <q-separator />
    <div style="height: 100px">.col</div>
  </div>
</template>
<script>
import { ref } from "vue";

export default {
  setup() {
    const leftDrawerOpen = ref(true);
    return {
      leftDrawerOpen,
    };
  },
};
</script>
