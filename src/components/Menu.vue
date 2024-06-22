<template>
  <n-menu :options="menuOptions" />
</template>

<style scoped></style>

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
} from "@vicons/ionicons5";

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
        "登陆"
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
          href: "#/unbindmacs",
        },
        "解绑MAC地址"
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
          href: "#/about",
        },
        "关于作者以及Tips"
      ),
    key: "about",
    icon: renderIcon(InformationCircleOutline),
  },
]

</script>