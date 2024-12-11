<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { EveryLoginData } from "./UserLoginLog.vue";
import { useLoadingBar, useMessage } from "naive-ui";
import { railStyle, mb2gb, min2hour } from "../helper";
import dayjs from "dayjs";
import MonthlyChart from "../components/MonthlyChart.vue";

const pop_message = useMessage();
const monthly_user_log = ref<Array<EveryLoginData>>([]);
// 把 start_date 设置为当前月第一天0点
const start_date = ref<number>(
  dayjs().startOf("month").startOf("day").valueOf()
);
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
const loadingBar = useLoadingBar();
const mb_gb_select = ref(false);

onMounted(() => {
  get_monthly_user_log();
});

const get_monthly_user_log = async () => {
  loadingBar.start();
  let res = await invoke("load_monthly_login_log", {
    startDate: Math.floor(start_date.value / 1000) + 8 * 3600,
    days: dayjs.unix(start_date.value / 1000).daysInMonth(),
  }).catch((err) => {
    pop_message.error(err);
    loadingBar.error();
  });
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
  loadingBar.finish();
};

const getBackgroundColor = (data_string: string) => {
  // const max = flow_max;
  // const min = 0;
  // const value = data / max * (max - min) + min;
  // const green = Math.round(255 * (1 - value / max));
  // const red = Math.round(255 * (value / max));
  // return `rgba(${red}, ${green}, 100, 0.2)`;
  let data = parseFloat(data_string);
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

const select_to_data = (item: EveryLoginData): string => {
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
  return (
    fieldMap[select_show_value.value].toFixed(
      select_show_value.value == "cost" ? 2 : 0
    ) || "0"
  );
};

const data_type = (): string => {
  if (select_show_value.value == "cost") {
    return "元";
  } else if (select_show_value.value == "used_duration") {
    return "分";
  } else if (mb_gb_select.value === true) {
    return "MB";
  } else {
    return "GB";
  }
};

const select_mb_or_gb = (value: string) => {
  if (
    select_show_value.value === "used_duration" ||
    select_show_value.value === "cost" ||
    mb_gb_select.value === true
  ) {
    return value;
  } else {
    return mb2gb(parseFloat(value));
  }
};
</script>

<template>
  <n-scrollbar style="max-height: 100vh">
    <div class="container">
      <n-h2 prefix="bar" type="success" style="margin-top: 15px">
        <n-text type="success"> 月度使用概览 </n-text>
      </n-h2>
      <n-date-picker
        v-model:value="start_date"
        type="month"
        clearable
        @update:value="get_monthly_user_log"
        style="margin-bottom: 10px"
      />
      <n-tabs type="segment" animated>
        <n-tab-pane name="calender" tab="日历">
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
              :style="{
                backgroundColor: getBackgroundColor(select_to_data(item)),
              }"
              ><n-popover trigger="hover">
                <template #trigger>
                  <p
                    style="margin: 3px; line-height: 1.5em; white-space: nowrap"
                  >
                    <b>{{ index + 1 }}日</b><br />
                    {{ select_mb_or_gb(select_to_data(item)) }}
                    {{ data_type() }}
                  </p>
                </template>
                <n-table :bordered="false" :single-line="false">
                  <thead>
                    <tr>
                      <th>ipv4 ⬇</th>
                      <th>ipv4 ⬆</th>
                      <th>ipv6 ⬇</th>
                      <th>ipv6 ⬆</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td>{{ mb2gb(item.ipv4_down) }} GB</td>
                      <td>{{ mb2gb(item.ipv4_up) }} GB</td>
                      <td>{{ mb2gb(item.ipv6_down) }} GB</td>
                      <td>{{ mb2gb(item.ipv6_up) }} GB</td>
                    </tr>
                    <tr>
                      <td>花费:</td>
                      <td>{{ item.cost.toFixed(2) }} 元</td>
                      <td>使用时长:</td>
                      <td>{{ min2hour(item.used_duration) }} h</td>
                    </tr>
                  </tbody>
                </n-table>
              </n-popover>
            </n-grid-item>
          </n-grid>
          <p>在使用概览上的东西选择：</p>
          <n-space>
            <n-select
              v-model:value="select_show_value"
              :options="select_show_options"
              style="width: 60vw"
            />
            <n-switch
              v-model:value="mb_gb_select"
              :rail-style="railStyle"
              class="my-switch"
              style="margin-top: calc((34px - 22px) / 2)"
            >
              <template #checked> MB </template>
              <template #unchecked> GB </template>
            </n-switch></n-space
          >
        </n-tab-pane>
        <n-tab-pane name="chart" tab="折线图">
          <MonthlyChart :monthly_user_log="monthly_user_log"></MonthlyChart>
        </n-tab-pane>
      </n-tabs>

      <p>关于统计信息：</p>
      <p>这里统计的每日情况与校园网后台一致，以下线时间为准。</p>
      <p>
        例如：你的手机连接了Wi-Fi，没断过，从第一天晚上8点用到了第二天凌晨4点，一共用了流量2GB，才断网，那么校园网后台才会统计一次信息，此时这2GB流量是算在第二天的。
      </p>
      <p>
        所以，这里的使用情况仅供参考，如果你每天都能在24点前断网，那么它也可能是准确的。
      </p>
    </div>
  </n-scrollbar>
</template>

<style scoped>
.container {
  overflow: auto;
  padding: 10px;
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
