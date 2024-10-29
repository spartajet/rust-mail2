import { createMemoryHistory, createRouter } from "vue-router";

// import HomeView from './HomeView.vue'
// import AboutView from './AboutView.vue'
import MainCalendar from "@/components/calendar/MainCalendar.vue";
import MainMail from "@/components/mails/MainMail.vue";
import MainSetting from "@/components/setting/MainSetting.vue";
import MainContact from "@/components/contact/MainContact.vue";
import MainTodo from "@/components/todo/MainTodo.vue";
const routes = [
  { path: "/", component: MainMail },
  { path: "/mail", component: MainMail },
  { path: "/mail/drafts", component: MainMail },
  { path: "/mail/sent", component: MainMail },
  { path: "/mail/trash", component: MainMail },
  { path: "/mail/starred", component: MainMail },
  { path: "/mail/important", component: MainMail },
  { path: "/mail/flagged", component: MainMail },
  { path: "/calendar", component: MainCalendar },
  { path: "/setting", component: MainSetting },
  { path: "/contact", component: MainContact },
  { path: "/todo", component: MainTodo },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
