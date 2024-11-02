<script setup lang="ts">
import { ref, computed, Ref, DefineComponent, onMounted } from "vue";
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
import OtherTools from "./pages/OtherTools.vue";
import { invoke } from "@tauri-apps/api/core";
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
  "/other_tools": OtherTools,
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

// Theme & background
const theme = ref<any | undefined>(undefined);
const themeMedia = window.matchMedia("(prefers-color-scheme: dark)");
const os_type = ref<number>(0);

onMounted(() => {
  get_os_type().then(() => {
    applyTheme(themeMedia.matches);
  });

  themeMedia.addEventListener("change", (event) => {
    applyTheme(event.matches);
  });
});

const applyTheme = (isDark: boolean) => {
  theme.value = isDark ? darkTheme : undefined;
  if (os_type.value % 2 === 0) {
    document.body.style.backgroundColor = isDark ? "#1f1f1f" : "#e6e6e6";
  }
};

const get_os_type = async () => {
  let res = await invoke("return_os_type");
  os_type.value = res as number;
};
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
