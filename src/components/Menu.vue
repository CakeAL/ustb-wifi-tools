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
          href: "#/about",
        },
        "关于作者"
      ),
    key: "about",
    icon: renderIcon(InformationCircleOutline),
  },
]

</script>