<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useLoadingBar, useMessage } from "naive-ui";
import { onMounted, ref } from "vue";
import SummaryTable from "../components/SummaryTable.vue";
import { railStyle, timestamp_format } from "../helper.ts";
// 用户在线日志摘要接口
export interface UserOnlineLogSummary {
  CHINANETDOWNFLOW: number; // ipv6 下
  INTERNETUPFLOW: number; // ipv4 上
  CHINANETUPFLOW: number; // IPv6 上
  COSTMONEY: number; // 计费金额
  COU: number; // 总条数
  TIME: number; // 使用时长 (分钟)
  INTERNETDOWNFLOW: number; // ipv4 下
  FLOW: number; // 使用流量
}

// 用户在线日志详细记录接口
export interface UserOnlineLogRow {
  area: number;
  chinanetDownFlow: number; // 下行ipv6
  chinanetUpFlow: number; // 上行ipv6
  costId: number;
  costMoney: number;
  costStyleId: number;
  extend: string;
  flddownflowIPV4: number; // 下行ipv4
  flddownflowIPV6: number; // 下行ipv6 数值上与上面 chinanetDownFlow 相等
  fldtotalflowIPV6: number;
  fldupflowIPV4: number; // 上行ipv4
  fldupflowIPV6: number; // 上行ipv6 数值上与 above chinanetUpFlow 相等
  flduserip1: string; // ipv6 地址
  flow: number;
  internetDownFlow: number;
  internetUpFlow: number;
  loginTime: number; // 时间戳
  logoutTime: number; // 时间戳
  macAddress: string;
  mainControlId: number;
  mutliGroupId: number;
  nasIp: string;
  nasPort: number;
  otherFlow: number;
  time: number; // 时长(分钟)
  userAddress: string;
  userGroupId: number;
  userId: number;
  userIp: string; // ipv4 地址
  userName: string;
  userPhone: string;
  userRealName: string;
}

// 用户在线日志主接口
export interface UserOnlineLog {
  summary: UserOnlineLogSummary;
  total: number;
  rows: UserOnlineLogRow[];
}

const pop_message = useMessage();
const date_range = ref<[number, number]>([Date.now(), Date.now()]);
const a_date = ref<number>(Date.now());
const user_online_log = ref<UserOnlineLog | null>(null);
const the_switch = ref(false);
const loadingBar = useLoadingBar();

const columns = [
  {
    title: "上线时间",
    key: "loginTime",
    render(row: UserOnlineLogRow) {
      return timestamp_format(row.loginTime);
    },
  },
  {
    title: "下线时间",
    key: "logoutTime",
    render(row: UserOnlineLogRow) {
      return timestamp_format(row.logoutTime);
    },
  },
  {
    title: "在线时长(分钟)",
    key: "time",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.time - row2.time,
  },
  {
    title: "消耗流量(MB)",
    key: "flow",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.flow - row2.flow,
  },
  {
    title: "花费(元)",
    key: "costMoney",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.costMoney - row2.costMoney,
  },
  {
    title: "ipv4 ⬆(MB)",
    key: "fldupflowIPV4",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.fldupflowIPV4 - row2.fldupflowIPV4,
  },
  {
    title: "ipv4 ⬇(MB)",
    key: "flddownflowIPV4",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.flddownflowIPV4 - row2.flddownflowIPV4,
  },
  {
    title: "ipv6 ⬆(MB)",
    key: "fldupflowIPV6",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.fldupflowIPV6 - row2.fldupflowIPV6,
  },
  {
    title: "ipv6 ⬇(MB)",
    key: "flddownflowIPV6",
    sorter: (row1: UserOnlineLogRow, row2: UserOnlineLogRow) =>
      row1.flddownflowIPV6 - row2.flddownflowIPV6,
  },
  { title: "ipv4 地址", key: "userIp" },
  { title: "ipv6 地址", key: "flduserip1" },
];

onMounted(() => {
  get_user_login_log();
});

const get_user_login_log = async () => {
  loadingBar.start();
  if (the_switch.value === true) {
    let res = await invoke("load_user_online_log", {
      startDate: Math.floor(date_range.value[0] / 1000) + 8 * 3600,
      endDate: Math.floor(date_range.value[1] / 1000) + 8 * 3600,
    }).catch((err) => {
      pop_message.error(err);
      loadingBar.error();
    });
    // console.log(res as string);
    user_online_log.value = JSON.parse(res as string);
  } else {
    let res = await invoke("load_user_online_log", {
      startDate: Math.floor(a_date.value / 1000) + 8 * 3600,
      endDate: Math.floor(a_date.value / 1000) + 8 * 3600,
    }).catch((err) => {
      pop_message.error(err);
      loadingBar.error();
    });
    // console.log(res as string);
    user_online_log.value = JSON.parse(res as string);
  }
  loadingBar.finish();
};
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 每日使用详情 </n-text>
    </n-h2>
    <n-grid x-gap="12" :cols="6">
      <n-gi>
        <n-switch
          v-model:value="the_switch"
          :rail-style="railStyle"
          style="margin-top: 6px"
        >
          <template #checked> 选很多天 </template>
          <template #unchecked> 选一天 </template>
        </n-switch>
      </n-gi>
      <n-gi span="5">
        <n-date-picker
          v-model:value="date_range"
          type="daterange"
          clearable
          @update:value="get_user_login_log"
          v-if="the_switch === true"
        />
        <n-date-picker
          v-model:value="a_date"
          type="date"
          clearable
          @update:value="get_user_login_log"
          v-else
        />
      </n-gi>
    </n-grid>
    <div v-if="user_online_log !== null" class="show-data">
      <SummaryTable
        title="该段时间"
        :summary="user_online_log.summary"
      ></SummaryTable>
      <br />
      <n-data-table
        :columns="columns"
        :data="user_online_log?.rows"
        :bordered="false"
        :max-height="600"
      />
    </div>
    <br />
    <n-card title="注意：" hoverable class="my-card">
      <n-p>貌似只有2026年1月15日之后的数据。</n-p>
      <!-- <n-p>本地数据不支持跨月选择。</n-p> -->
    </n-card>
  </div>
</template>

<style scoped>
.show-data {
  margin-top: 10px;
}
</style>
