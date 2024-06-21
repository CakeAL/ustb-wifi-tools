<script setup lang="ts">
import { ref, computed, Ref, DefineComponent } from "vue";

// routers
import Login from "./pages/Login.vue";
import About from "./pages/About.vue";
import NotFound from "./pages/NotFound.vue";
import UserInfo from "./pages/UserInfo.vue";
import MonthPay from "./pages/MonthPay.vue";
import UserLoginLog from "./pages/UserLoginLog.vue"
import UnbindMacs from "./pages/UnbindMacs.vue";
import SpeedTest from "./pages/SpeedTest.vue"
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
</script>

<template>
  <n-message-provider>
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
  </n-message-provider>
</template>

<style scoped></style>
