<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { EveryLoginData } from "./UserLoginLog.vue";
import { useMessage } from "naive-ui";

import dayjs from "dayjs";

const pop_message = useMessage();
const monthly_user_log = ref<Array<EveryLoginData>>([]);
const start_date = ref<number>(Date.now());
const the_week_of_first_day = ref<Array<number>>([]);
const refresh = ref(true);
// let flow_max = 0;

const get_monthly_user_log = async () => {
  let res = await invoke("load_monthly_login_log", {
    startDate: Math.floor(start_date.value / 1000) + 8 * 3600,
    days: dayjs.unix(start_date.value / 1000).daysInMonth(),
  }).catch((err) => pop_message.error(err));
  monthly_user_log.value = JSON.parse(res as string);
  // // 找出当月使用流量最大的
  // flow_max = -1;
  // monthly_user_log.value.forEach((value) => {
  //   if (value.ipv4_down > flow_max) {
  //     flow_max = value.ipv4_down;
  //   }
  // })
  // 该月1日是星期几？前面空余几个格子
  the_week_of_first_day.value = [];
  for (let i = 0; i < dayjs.unix(start_date.value / 1000).day(); i++) {
    the_week_of_first_day.value.push(i);
  }
  // console.log(monthly_user_log.value);
  // 刷新组件
  refresh.value = !refresh.value;
};

const getBackgroundColor = (ipv4_down: number) => {
  // const max = flow_max;
  // const min = 0;
  // const value = ipv4_down / max * (max - min) + min;
  // const green = Math.round(255 * (1 - value / max));
  // const red = Math.round(255 * (value / max));
  // return `rgba(${red}, ${green}, 100, 0.2)`;
  if (ipv4_down < 1000) {
    return "rgba(55, 80, 147, 0.5)";
  } else if (ipv4_down < 2000) {
    return "rgba(78, 112, 175, 0.5)";
  } else if (ipv4_down < 3000) {
    return "rgba(112, 145, 199, 0.5)";
  } else if (ipv4_down < 4000) {
    return "rgba(158, 188, 219, 0.5)";
  } else if (ipv4_down < 5000) {
    return "rgba(200, 214, 231, 0.5)";
  } else if (ipv4_down < 7000) {
    return "rgba(236, 208, 180, 0.5)";
  } else if (ipv4_down < 8500) {
    return "rgba(219, 162, 125, 0.5)";
  } else if (ipv4_down < 10000) {
    return "rgba(193, 109, 88, 0.5)";
  } else if (ipv4_down < 15000) {
    return "rgba(161, 61, 59, 0.5)";
  } else {
    return "rgba(131, 26, 33, 0.5)";
  }
};
</script>

<template>
  <div class="container">
    <n-date-picker
      v-model:value="start_date"
      type="month"
      clearable
      @update:value="get_monthly_user_log"
    />
    <n-grid :x-gap="12" :y-gap="8" :cols="7" :key="refresh">
      <n-grid-item>日</n-grid-item>
      <n-grid-item>一</n-grid-item>
      <n-grid-item>二</n-grid-item>
      <n-grid-item>三</n-grid-item>
      <n-grid-item>四</n-grid-item>
      <n-grid-item>五</n-grid-item>
      <n-grid-item>六</n-grid-item>
      <n-grid-item
        v-for="(, index) in the_week_of_first_day"
        :key="index"
        class="gray"
      >
      </n-grid-item>
      <n-grid-item
        v-for="(item, index) in monthly_user_log"
        :key="index"
        class="day"
        :style="{ backgroundColor: getBackgroundColor(item.ipv4_down) }"
      >
        {{ index + 1 }}<br />
        {{ item.ipv4_down.toFixed(0) }} MB
      </n-grid-item>
    </n-grid>
    <p>关于统计信息：</p>
    <p>这里统计的每日情况与校园网后台一致，以下线时间为准。</p>
    <p>
      例如：你的手机连接了Wi-Fi，没断过，从第一天晚上8点用到了第二天凌晨4点，一共用了流量2GB，才断网，那么校园网后台才会统计一次信息，此时这2GB流量是算在第二天的。
    </p>
    <p>
      所以，这里的使用情况仅供参考，如果你每天都能在24点前断网，那么它也可能是准确的。
    </p>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
}
.gray {
  height: 50px;
  background-color: rgb(80, 80, 80, 0.2);
}
.day {
  height: 50px;
}
</style>
