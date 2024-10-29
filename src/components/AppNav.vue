<template>
  <v-navigation-drawer
    class="bg-deep-purple"
    expand-on-hover
    rail
    width="250"
    permanent
  >
    <v-list density="compact" style="padding: 4px;">
      <v-list-item style="padding-left: 8px;"
        prepend-avatar="https://randomuser.me/api/portraits/women/85.jpg"
        subtitle="sandra_a88@gmailcom"
        title="Sandra Adams"
      ></v-list-item>
    </v-list>
    <v-divider :thickness="1" class="border-opacity-100"></v-divider>
    <v-list density="compact" nav link style="padding: 4px;">
      <v-list-item
        prepend-icon="mdi-inbox-arrow-down"
        title="Inbox"
        value="Inbox" @click="$router.push('/mail')"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-account-star"
        title="Starred"
        value="Starred"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-flag"
        title="Flag"
        value="Flag"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-book-account"
        title="BookAccount"
        value="BookAccount"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-fountain-pen-tip"
        title="Drafts"
        value="Drafts"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-calendar-month"
        title="Calendar"
        value="Calendar" @click="$router.push('/calendar')"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-checkbox-multiple-outline"
        title="Todo"
        value="Todo"
      ></v-list-item>
      <v-divider :thickness="1" class="border-opacity-100"></v-divider>

    </v-list>
    <template v-slot:append>
      <v-divider :thickness="1" class="border-opacity-100"></v-divider>
      <v-list density="compact" nav>
        <v-list-item
          prepend-icon="mdi-plus"
          title="Add Account"
          value="AddAccount"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-delete"
          title="Trash"
          value="Trash" style="padding-top: 8px;"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-tune"
          title="Setting"
          value="Setting" @click="$router.push('/setting')"
        ></v-list-item>
      </v-list>
    </template>
  </v-navigation-drawer>
</template>
<script setup lang="ts"></script>
<style>
.v-list-item {
  padding: 4px 9px;
}
.v-list-item__prepend{
  height: 20px;
}
.v-icon--size-default{
  font-size: 25px !important;
}
.v-avatar{
  height: 32px !important;
  width: 32px !important;
}
</style>
