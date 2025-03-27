<script setup lang="ts">
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";
import { darkTheme } from "naive-ui";
import { computed, DefineComponent, onMounted, Ref, ref } from "vue";
import { check_update, download_percent, is_download } from "./update";

// routers
import About from "./pages/About.vue";
import ElectricBill from "./pages/ElectricBill.vue";
import Login from "./pages/Login.vue";
import MonthlyUserLog from "./pages/MonthlyUserLog.vue";
import MonthPay from "./pages/MonthPay.vue";
import NotFound from "./pages/NotFound.vue";
import OtherTools from "./pages/OtherTools.vue";
import SpeedTest from "./pages/SpeedTest.vue";
import UnbindMacs from "./pages/UnbindMacs.vue";
import UserInfo from "./pages/UserInfo.vue";
import UserLoginLog from "./pages/UserLoginLog.vue";
import { store } from "./store";

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
  "/electric_bill": ElectricBill,
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
const currentPlatform = platform();
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
  let res =
    (await invoke("load_setting").catch((err) => console.log(err))) as string;
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
            background-image: url("${
          convertFileSrc(
            settings.background_image_path,
          )
        }"); 
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

// download & onedrive & userName
onMounted(() => {
  check_update(false);
  load_collapse();
  getCurUserName();
});

const getCurUserName = async () => {
  store.userName = await invoke("get_current_user_name");
};

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
              <Menu
                :collapsed="collapsed"
                :currentPlatform="currentPlatform"
              ></Menu>
            </n-layout-sider>
            <n-layout>
              <n-scrollbar style="max-height: 100vh">
                <Transition name="slide-up" mode="out-in">
                  <component
                    :is="currentView"
                    style="padding: 10px; scrollbar-width: 0"
                  />
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

  <div
    v-if="currentPlatform === 'macos'"
    data-tauri-drag-region
    class="title-bar"
  >
  </div>
</template>

<style scoped>
.title-bar {
  position: fixed;
  top: 0;
  left: 0;
  height: 28px;
  width: 100%;
  /* box-shadow: inset 0px -0.5px 0px rgba(255, 255, 255, 0.5); */
  z-index: 10000;
}

.download-progress {
  position: fixed;
  bottom: 0;
  left: 0;
  width: calc(100vw - 20px);
  padding: 10px;
}

.slide-up-enter-active {
  transition: all 0.2s ease;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(150px);
}
</style>
