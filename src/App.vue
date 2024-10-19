<script setup lang="ts">
import { ref, computed, Ref, DefineComponent } from "vue";
import { darkTheme } from "naive-ui";

// routers
import Login from "./pages/Login.vue";
import About from "./pages/About.vue";
import NotFound from "./pages/NotFound.vue";
import UserInfo from "./pages/UserInfo.vue";
import MonthPay from "./pages/MonthPay.vue";
import UserLoginLog from "./pages/UserLoginLog.vue";
import UnbindMacs from "./pages/UnbindMacs.vue";
import SpeedTest from "./pages/SpeedTest.vue";
import MonthlyUserLog from "./pages/MonthlyUserLog.vue";
type RouteComponent = DefineComponent<{}, {}, any>;

interface Routes {
  [key: string]: RouteComponent;
}

const routes: Routes = {
  "/": Login,
  "/about": About,
  "/userinfo": UserInfo,
  "/monthpay": MonthPay,
  "/userloginlog": UserLoginLog,
  "/unbindmacs": UnbindMacs,
  "/speedtest": SpeedTest,
  "/monthly_user_log": MonthlyUserLog,
};

// Ref for current path
const currentPath: Ref<string> = ref(window.location.hash);
window.addEventListener("hashchange", () => {
  currentPath.value = window.location.hash;
});

// Computed property for the current view
const currentView = computed((): RouteComponent => {
  return routes[currentPath.value.slice(1) || "/"] || NotFound;
});

// Theme
const theme = ref<any | undefined>(undefined);
const themeMedia = window.matchMedia("(prefers-color-scheme: dark)");
if (themeMedia.matches) {
  theme.value = darkTheme;
} else {
  theme.value = undefined;
}
themeMedia.addEventListener("change", (event) => {
  if (event.matches) {
    theme.value = darkTheme;
  } else {
    theme.value = undefined;
  }
});
</script>

<template>
  <n-message-provider>
    <n-loading-bar-provider>
      <n-config-provider :theme="theme">
        <div class="container">
          <n-split
            direction="horizontal"
            style="height: 100vh"
            max="300px"
            min="200px"
            default-size="200px"
          >
            <template #1>
              <Menu />
            </template>
            <template #2>
              <component :is="currentView" />
            </template>
          </n-split>
        </div>
      </n-config-provider>
    </n-loading-bar-provider>
  </n-message-provider>
</template>

<style scoped></style>
