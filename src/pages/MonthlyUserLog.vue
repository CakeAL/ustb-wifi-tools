<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import dayjs from "dayjs";
import { useLoadingBar, useMessage } from "naive-ui";
import { onMounted, ref } from "vue";
import MonthlyChart from "../components/MonthlyChart.vue";
import SummaryTable from "../components/SummaryTable.vue";
import { mb2gb, min2hour, railStyle } from "../helper";
import { UserOnlineLog, UserOnlineLogRow } from "./UserOnlineLog.vue";

const pop_message = useMessage();
const user_online_log = ref<UserOnlineLog | null>(null);
const daily_log = ref<Array<UserOnlineLogRow>>([]);
// 把 start_date 设置为当前月第一天0点
const start_date = ref<number>(
  dayjs().startOf("month").startOf("day").valueOf(),
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

onMounted(() => {
  get_monthly_user_log();
});

const get_monthly_user_log = async () => {
  loadingBar.start();
  let startTimestamp = Math.floor(start_date.value / 1000) + 8 * 3600;
  let days = dayjs.unix(start_date.value / 1000).daysInMonth();
  let endTimestamp = startTimestamp + 24 * 3600 * days;
  let res = await invoke("load_user_online_log", {
    startDate: startTimestamp,
    endDate: endTimestamp,
  }).catch((err) => {
    pop_message.error(err);
    loadingBar.error();
  });
  user_online_log.value = JSON.parse(res as string);
  daily_log.value = [];
  for (let i = 0; i < days; i++) {
    let sum: UserOnlineLogRow = {
      area: 0,
      chinanetDownFlow: 0,
      chinanetUpFlow: 0,
      costId: 0,
      costMoney: 0,
      costStyleId: 0,
      extend: "",
      flddownflowIPV4: 0,
      flddownflowIPV6: 0,
      fldtotalflowIPV6: 0,
      fldupflowIPV4: 0,
      fldupflowIPV6: 0,
      flduserip1: "",
      flow: 0,
      internetDownFlow: 0,
      internetUpFlow: 0,
      loginTime: 0,
      logoutTime: 0,
      macAddress: "",
      mainControlId: 0,
      mutliGroupId: 0,
      nasIp: "",
      nasPort: 0,
      otherFlow: 0,
      time: 0,
      userAddress: "",
      userGroupId: 0,
      userId: 0,
      userIp: "",
      userName: "",
      userPhone: "",
      userRealName: ""
    };
    user_online_log.value?.rows.filter((item) => {
      return item.logoutTime >= startTimestamp * 1000 + i * 24 * 3600 * 1000
        && item.logoutTime < startTimestamp * 1000 + (i + 1) * 24 * 3600 * 1000;
    }).forEach((item) => {
      sum.costMoney += item.costMoney;
      sum.flddownflowIPV4 += item.flddownflowIPV4;
      sum.fldupflowIPV4 += item.fldupflowIPV4;
      sum.flddownflowIPV6 += item.flddownflowIPV6;
      sum.fldupflowIPV6 += item.fldupflowIPV6;
      sum.time += item.time;
      sum.flow += item.flow;
    });
    daily_log.value.push(sum);
  }

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

const select_to_data = (item: UserOnlineLogRow): string => {
  const fieldMap: { [key: string]: number } = {
    ipv4_down: item.flddownflowIPV4,
    ipv4_up: item.fldupflowIPV4,
    ipv6_down: item.flddownflowIPV6,
    ipv6_up: item.fldupflowIPV6,
    ipv4: item.flddownflowIPV4 + item.fldupflowIPV4,
    ipv6: item.flddownflowIPV6 + item.fldupflowIPV6,
    all: item.flddownflowIPV4 + item.fldupflowIPV4 + item.flddownflowIPV6 + item.fldupflowIPV6,
    cost: item.costMoney,
    used_duration: item.time,
  };
  return (
    fieldMap[select_show_value.value].toFixed(
      select_show_value.value == "cost" ? 2 : 0,
    ) || "0"
  );
};

const data_type = (): string => {
  if (select_show_value.value == "cost") {
    return "元";
  } else if (select_show_value.value == "used_duration") {
    return "分";
  } else { return "" }
};

const select_mb_or_gb = (value: string) => {
  if (
    select_show_value.value === "used_duration"
    || select_show_value.value === "cost"
  ) {
    return value;
  } else {
    return mb2gb(parseFloat(value));
  }
};
</script>

<template>
  <div>
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 月度使用概览 </n-text>
    </n-h2>
    <n-date-picker v-model:value="start_date" type="month" clearable @update:value="get_monthly_user_log" />
    <n-tabs type="segment" animated style="margin-top: 5px">
      <n-tab-pane name="calender" tab="日历" style="padding-top: 8px">
        <n-grid :x-gap="12" :y-gap="8" :cols="7" :key="refresh">
          <n-grid-item class="gray">
            <p>日</p>
          </n-grid-item>
          <n-grid-item class="gray">
            <p>一</p>
          </n-grid-item>
          <n-grid-item class="gray">
            <p>二</p>
          </n-grid-item>
          <n-grid-item class="gray">
            <p>三</p>
          </n-grid-item>
          <n-grid-item class="gray">
            <p>四</p>
          </n-grid-item>
          <n-grid-item class="gray">
            <p>五</p>
          </n-grid-item>
          <n-grid-item class="gray">
            <p>六</p>
          </n-grid-item>
          <n-grid-item v-for="(_, index) in the_week_of_first_day" :key="index" class="gray">
          </n-grid-item>
          <n-grid-item v-for="(item, index) in daily_log" :key="index" class="day" :style="{
            backgroundColor: getBackgroundColor(
              select_to_data(item),
            ),
          }
            "><n-popover trigger="hover">
              <template #trigger>
                <p style="margin: 3px; line-height: 1.5em; white-space: nowrap">
                  <b>{{ index + 1 }}日</b>
                  <br />
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
                    <td>{{ mb2gb(item.flddownflowIPV4) }}</td>
                    <td>{{ mb2gb(item.fldupflowIPV4) }}</td>
                    <td>{{ mb2gb(item.flddownflowIPV6) }}</td>
                    <td>{{ mb2gb(item.fldupflowIPV6) }}</td>
                  </tr>
                  <tr>
                    <td>花费:</td>
                    <td>{{ item.costMoney }} 元</td>
                    <td>使用时长:</td>
                    <td>{{ min2hour(item.time) }} h</td>
                  </tr>
                </tbody>
              </n-table>
            </n-popover>
          </n-grid-item>
        </n-grid>
        <SummaryTable :summary="user_online_log?.summary"></SummaryTable>
        <n-grid x-gap="12" :cols="4" style="margin-top: 8px">
          <n-gi span="2"><n-p style="line-height: 34px">选择显示在日历上的内容：</n-p></n-gi>
          <n-gi span="2"><n-select v-model:value="select_show_value" :options="select_show_options" /></n-gi>
        </n-grid>
      </n-tab-pane>
      <n-tab-pane name="chart" tab="折线图" style="padding-top: 8px">
        <MonthlyChart :daily_log="daily_log"></MonthlyChart>
      </n-tab-pane>
    </n-tabs>
    <n-card title="关于统计信息：" hoverable class="my-card">
      <p>这里统计的每日情况与校园网后台一致，以下线时间为准。</p>
      <p>
        例如：你的手机连接了Wi-Fi，没断过，从第一天晚上8点用到了第二天凌晨4点，一共用了流量2GB，才断网，那么校园网后台才会统计一次信息，此时这2GB流量是算在第二天的。
      </p>
      <p>
        所以，这里的使用情况仅供参考，如果你每天都能在24点前断网，那么它也可能是准确的。
      </p>
    </n-card>
  </div>
</template>

<style scoped>
.gray {
  height: 50px;
  text-align: center;
  border-radius: 5px;
  background-color: rgb(80, 80, 80, 0.2);
}

.day {
  height: 50px;
  border-radius: 5px;
  transition: all 0.3s ease-in-out;
}

.day:hover {
  box-shadow:
    rgba(127, 231, 196, 0.4) 0 3px,
    rgba(127, 231, 196, 0.3) 0 6px,
    rgba(127, 231, 196, 0.2) 0 9px,
    rgba(127, 231, 196, 0.1) 0 12px,
    rgba(127, 231, 196, 0.05) 0 15px;
}
</style>
