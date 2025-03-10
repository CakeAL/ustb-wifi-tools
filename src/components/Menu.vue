<template>
  <div v-if="os_type === 3" class="title-bar"></div>
  <n-menu data-tauri-drag-region :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions"
    class="my-menu" />
</template>

<style scoped>
.title-bar {
  height: 28px;
  width: 100%;
}

.my-menu {
  overflow: auto;
}
</style>

<script setup lang="ts">
import { h, Component } from "vue";
import { NIcon } from "naive-ui";
import type { MenuOption } from "naive-ui";
import {
  LogInOutline,
  InformationCircleOutline,
  ListOutline,
  CashOutline,
  BookOutline,
  BrowsersOutline,
  SpeedometerOutline,
  CalendarOutline,
  BuildOutline,
} from "@vicons/ionicons5";

defineProps<{
  collapsed: boolean;
  os_type: number;
}>();

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        "a",
        {
          href: "#/",
        },
        "账号管理"
      ),
    key: "login",
    icon: renderIcon(LogInOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/userinfo",
        },
        "当前账号使用详情"
      ),
    key: "userinfo",
    icon: renderIcon(ListOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/monthpay",
        },
        "年度扣费账单"
      ),
    key: "monthpay",
    icon: renderIcon(CashOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/userloginlog",
        },
        "每日使用详情"
      ),
    key: "userloginlog",
    icon: renderIcon(BookOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/monthly_user_log",
        },
        "月度使用概览"
      ),
    key: "monthly_user_log",
    icon: renderIcon(CalendarOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/unbindmacs",
        },
        "解绑 MAC 地址"
      ),
    key: "unbindmacs",
    icon: renderIcon(BrowsersOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/speedtest",
        },
        "测个速"
      ),
    key: "speedtest",
    icon: renderIcon(SpeedometerOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/other_tools",
        },
        "其他小工具"
      ),
    key: "other_tools",
    icon: renderIcon(BuildOutline),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/about",
        },
        "关于作者以及 Tips"
      ),
    key: "about",
    icon: renderIcon(InformationCircleOutline),
  },
];
</script>
