<template>
  <q-layout view="lHh LpR lFf">
    <q-drawer
      show-if-above
      v-model="leftDrawerOpen"
      side="left"
      behavior="desktop"
      bordered
      :width="300"
    >
      <div class="column" style="height: 100%">
        <div class="row" style="height: 80px">
          <q-item clickable v-ripple class="q-pa-sm">
            <q-item-section side>
              <q-avatar rounded size="50px">
                <img src="https://cdn.quasar.dev/img/avatar.png" />
                <q-badge floating color="primary">{{
                  newMessageCount
                }}</q-badge>
              </q-avatar>
            </q-item-section>
            <q-item-section class="q-ma-none" style="margin-top: 0px">
              <q-item-label style="font-size: 22px; font-weight: 500"
                >Dr.Guo</q-item-label
              >
              <q-item-label>
                <q-badge color="warning" class="q-mr-xs">
                  12 <q-icon name="star" color="white" class="q-ml-xs" />
                </q-badge>
                <q-badge color="secondary" class="q-mr-xs">
                  12 <q-icon name="alarm" color="white" class="q-ml-xs" />
                </q-badge>
                <q-badge color="accent" class="q-mr-xs">
                  12 <q-icon name="flag" color="white" class="q-ml-xs" />
                </q-badge>
              </q-item-label>
            </q-item-section>
          </q-item>
        </div>
        <q-separator class="q-ml-xs q-mr-xs" />
        <div class="col q-pt-sm">
          <q-list class="rounded-borders">
            <q-expansion-item
              dense
              dense-toggle
              expand-separator
              icon="perm_identity"
              label="Accounts"
              header-class="text-primary text-weight-bold "
            >
              <q-card>
                <q-card-section class="q-pl-sm q-pt-none q-pr-none q-pb-none">
                  <q-list dense>
                    <q-item
                      clickable
                      class="q-pa-none q-ma-none"
                      v-ripple
                      v-for="account in mailAccounts"
                      :key="account.account"
                    >
                      <q-item-section>
                        <div class="row">
                          <q-icon class="col-2" :name="account.avatar" />
                          <div class="col-10 text-weight-bold">
                            {{ account.account }}
                          </div>
                        </div>
                      </q-item-section>
                    </q-item>
                  </q-list>
                </q-card-section>
              </q-card>
            </q-expansion-item>

            <q-expansion-item
              dense
              dense-toggle
              expand-separator
              icon="folder"
              label="Folders"
              header-class="text-secondary text-weight-bold"
            >
              <q-card>
                <q-card-section>
                  Lorem ipsum dolor sit amet, consectetur adipisicing elit.
                  Quidem, eius reprehenderit eos corrupti commodi magni quaerat
                  ex numquam, dolorum officiis modi facere maiores architecto
                  suscipit iste eveniet doloribus ullam aliquid.
                </q-card-section>
              </q-card>
            </q-expansion-item>

            <q-expansion-item
              dense
              dense-toggle
              expand-separator
              icon="notifications_active"
              label="Notifycation"
              header-class="text-purple text-weight-bold"
            >
              <q-card>
                <q-card-section>
                  Lorem ipsum dolor sit amet, consectetur adipisicing elit.
                  Quidem, eius reprehenderit eos corrupti commodi magni quaerat
                  ex numquam, dolorum officiis modi facere maiores architecto
                  suscipit iste eveniet doloribus ullam aliquid.
                </q-card-section>
              </q-card>
            </q-expansion-item>
          </q-list>
        </div>
      </div>
      <!-- <div style="height: 100px">1 of 2</div> -->
    </q-drawer>

    <q-page-container>
      <div class="row no-wrap">
        <div style="width: 200px"></div>
        <div class="col-12">
          <router-view />
        </div>
      </div>
    </q-page-container>
  </q-layout>
</template>

<script>
import { ref } from "vue";

export default {
  setup() {
    const leftDrawerOpen = ref(true);
    const newMessageCount = ref(3);
    const starMessage = ref(false);
    const flagMessage = ref(false);
    const notificationMessage = ref(false);
    const mailAccounts = ref([
      {
        name: "Private",
        avatar: "school",
        account: "guoxiaozhong1990@outlook.com",
        newMessageCount: 3,
        starMessage: 12,
        flagMessage: 12,
        notificationMessage: 12,
      },
      {
        name: "Dr.Guo",
        avatar: "apartment",
        account: "guoxiaozhong@zstu.edu.cn",
        newMessageCount: 3,
        starMessage: 12,
        flagMessage: 12,
        notificationMessage: 12,
      },
    ]);

    return {
      leftDrawerOpen,
      newMessageCount,
      starMessage,
      flagMessage,
      notificationMessage,
      mailAccounts,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value;
      },
    };
  },
};
</script>

<style lang="sass" scoped>
.column > div
.column + .column
  margin-top: 1rem

.mail_list_icon
  size: 20px

.q-list--dense > .q-item,
.q-item--dense
  min-height: 32px
  padding: 2px 2px
</style>
