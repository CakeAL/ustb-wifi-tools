<script setup lang="ts">
import { ref, computed, Ref, DefineComponent, onMounted } from "vue";
import { darkTheme } from "naive-ui";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { check_update, is_download, download_percent } from "./update";

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
theme.value = themeMedia.matches ? darkTheme : undefined;

onMounted(() => {
  get_os_type()
    .then(() => {
      applyBackgroundColor(themeMedia.matches);
    })
    .then(apply_background);
  themeMedia.addEventListener("change", (event) => {
    theme.value = event.matches ? darkTheme : undefined;
    applyBackgroundColor(event.matches);
  });
});

const applyBackgroundColor = (isDark: boolean) => {
  if (os_type.value % 2 === 0) {
    document.body.style.backgroundColor = isDark ? "#1f1f1f" : "#e6e6e6";
  }
};

const get_os_type = async () => {
  let res = await invoke("return_os_type");
  os_type.value = res as number;
};

const apply_background = async () => {
  let res = (await invoke("load_setting").catch((err) =>
    console.log(err)
  )) as string;
  if (res.length > 0) {
    let settings = JSON.parse(res);
    // 如果存在 background 路径的情况下
    if (settings.background_image_path !== null) {
      let html = document.querySelector("body");
      if (html) {
        let style = document.createElement("style");
        style.innerHTML = `
          body { 
            background-color: rgba(0, 0, 0, 0); 
          } 
          body::before { 
            content: ''; 
            position: absolute; 
            top: 0; 
            left: 0; 
            right: 0; 
            bottom: 0; 
            background-image: url("${convertFileSrc(
              settings.background_image_path
            )}"); 
            background-size: cover; 
            background-position: center; 
            filter: blur(${settings.background_blur}px);
            opacity: ${settings.background_transparence / 100}; 
            z-index: -1; 
          }
        `;
        document.head.appendChild(style);
      }
    }
  }
};

// download & onedrive
onMounted(() => {
  check_update(false);
  load_collapse();
});

// sider
const collapsed = ref(true);
const load_collapse = async () => {
  let res = (await invoke("load_setting")) as string;
  if (res.length > 0) {
    let settings = JSON.parse(res);
    collapsed.value = settings.collapsed;
  }
};
const collapse = async (value: boolean) => {
  collapsed.value = value;
  await invoke("collapse", { value });
};
</script>

<template>
  <n-modal-provider>
    <n-message-provider>
      <n-loading-bar-provider>
        <n-config-provider :theme="theme">
          <n-layout has-sider>
            <n-layout-sider
              bordered
              collapse-mode="width"
              :collapsed-width="64"
              :width="200"
              :collapsed="collapsed"
              show-trigger
              @collapse="collapse(true)"
              @expand="collapse(false)"
            >
              <Menu :collapsed="collapsed"></Menu>
            </n-layout-sider>
            <n-layout>
              <n-scrollbar style="max-height: 100vh">
              <Transition name="slide-up" mode="out-in">
                <component :is="currentView" style="padding: 10px; scrollbar-width: 0;"/>
              </Transition>
              </n-scrollbar>
            </n-layout>
          </n-layout>
        </n-config-provider>
      </n-loading-bar-provider>
    </n-message-provider>
  </n-modal-provider>
  <n-progress
    type="line"
    :percentage="download_percent"
    status="success"
    indicator-placement="inside"
    processing
    class="download-progress"
    v-if="is_download"
  />
</template>

<style scoped>
.download-progress {
  position: fixed;
  bottom: 0;
  left: 0;
  width: calc(100vw - 20px);
  padding: 10px;
}

.slide-up-enter-active {
  transition: all 0.20s ease-out;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(150px);
}
</style>
