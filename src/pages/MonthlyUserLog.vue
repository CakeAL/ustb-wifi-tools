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
const select_show_value = ref<string>("ipv4_down");
const select_show_options = [
  {
    label: "ipv4 下行（校园网只计费这个）",
    value: "ipv4_down",
  },
  {
    label: "ipv4 上行",
    value: "ipv4_up",
  },
  {
    label: "ipv6 下行",
    value: "ipv6_down",
  },
  {
    label: "ipv6 上行",
    value: "ipv6_up",
  },
  {
    label: "ipv4 上下行",
    value: "ipv4",
  },
  {
    label: "ipv6 上下行",
    value: "ipv6",
  },
  {
    label: "当天总共的流量（包含ipv4和ipv6上下行）",
    value: "all",
  },
  {
    label: "使用时长",
    value: "used_duration",
  },
  {
    label: "花费",
    value: "cost",
  },
];
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

const getBackgroundColor = (data: number) => {
  // const max = flow_max;
  // const min = 0;
  // const value = data / max * (max - min) + min;
  // const green = Math.round(255 * (1 - value / max));
  // const red = Math.round(255 * (value / max));
  // return `rgba(${red}, ${green}, 100, 0.2)`;
  if (data < 1000) {
    return "rgba(55, 80, 147, 0.5)";
  } else if (data < 2000) {
    return "rgba(78, 112, 175, 0.5)";
  } else if (data < 3000) {
    return "rgba(112, 145, 199, 0.5)";
  } else if (data < 4000) {
    return "rgba(158, 188, 219, 0.5)";
  } else if (data < 5000) {
    return "rgba(200, 214, 231, 0.5)";
  } else if (data < 7000) {
    return "rgba(236, 208, 180, 0.5)";
  } else if (data < 8500) {
    return "rgba(219, 162, 125, 0.5)";
  } else if (data < 10000) {
    return "rgba(193, 109, 88, 0.5)";
  } else if (data < 15000) {
    return "rgba(161, 61, 59, 0.5)";
  } else {
    return "rgba(131, 26, 33, 0.5)";
  }
};

const select_to_data = (item: EveryLoginData): number => {
  const fieldMap: { [key: string]: number } = {
    ipv4_down: item.ipv4_down,
    ipv4_up: item.ipv4_up,
    ipv6_down: item.ipv6_down,
    ipv6_up: item.ipv6_up,
    ipv4: item.ipv4_down + item.ipv4_up,
    ipv6: item.ipv6_down + item.ipv6_up,
    all: item.ipv4_down + item.ipv4_up + item.ipv6_down + item.ipv6_up,
    cost: item.cost,
    used_duration: item.used_duration,
  };
  return fieldMap[select_show_value.value] || 0;
};

const data_type = (): string => {
  if (select_show_value.value == "cost") {
    return "元";
  } else if (select_show_value.value == "used_duration") {
    return "分";
  } else {
    return "MB";
  }
};
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 月度使用概览 </n-text>
    </n-h2>
    <n-date-picker
      v-model:value="start_date"
      type="month"
      clearable
      @update:value="get_monthly_user_log"
    />
    <br />
    <n-grid :x-gap="12" :y-gap="8" :cols="7" :key="refresh">
      <n-grid-item class="gray"><p>日</p></n-grid-item>
      <n-grid-item class="gray"><p>一</p></n-grid-item>
      <n-grid-item class="gray"><p>二</p></n-grid-item>
      <n-grid-item class="gray"><p>三</p></n-grid-item>
      <n-grid-item class="gray"><p>四</p></n-grid-item>
      <n-grid-item class="gray"><p>五</p></n-grid-item>
      <n-grid-item class="gray"><p>六</p></n-grid-item>
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
        :style="{ backgroundColor: getBackgroundColor(select_to_data(item)) }"
      >
        <p style="margin: 3px; line-height: 1.5em; white-space: nowrap">
          <b>{{ index + 1 }}日</b><br />
          {{ select_to_data(item).toFixed(0) }} {{ data_type() }}
        </p>
      </n-grid-item>
    </n-grid>
    <p>在使用概览上的东西选择：</p>
    <n-select
      v-model:value="select_show_value"
      :options="select_show_options"
    />
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
  margin: 5px;
}
.gray {
  height: 50px;
  text-align: center;
  border-radius: 5px;
  background-color: rgb(80, 80, 80, 0.2);
}
.day {
  height: 50px;
  border-radius: 5px;
}
</style>
