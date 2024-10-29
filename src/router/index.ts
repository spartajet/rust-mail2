import { createMemoryHistory, createRouter } from "vue-router";

// import HomeView from './HomeView.vue'
// import AboutView from './AboutView.vue'
import MainCalendar from "@/components/calendar/MainCalendar.vue";
import MainMail from "@/components/mails/MainMail.vue";
import SettingMain from "@/components/setting/SettingMain.vue";

const routes = [
  { path: "/", component: MainMail },
  { path: "/mail", component: MainMail },
  { path: "/calendar", component: MainCalendar },
  { path: "/setting", component: SettingMain },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
