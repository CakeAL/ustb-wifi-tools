<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useLoadingBar, useMessage } from "naive-ui";
import dayjs from "dayjs";
import { railStyle, min2hour } from "../helper";

interface UserLoginLog {
  ipv4_up: number;
  ipv4_down: number;
  ipv6_up: number;
  ipv6_down: number;
  used_flow: number; // 实际上就是 ipv4 下行
  cost: number;
  used_duration: number;
  every_login_data: EveryLoginData[];
}

export interface EveryLoginData {
  online_time: number; // 时间戳，UTC
  offline_time: number;
  used_duration: number;
  used_flow: number; // 实际上就是 ipv4 下行
  cost: number;
  ipv4_up: number;
  ipv4_down: number;
  ipv6_up: number;
  ipv6_down: number;
  ipv4_addr: string;
  ipv6_addr: string;
}

const pop_message = useMessage();
const date_range = ref<[number, number]>([Date.now(), Date.now()]);
const a_date = ref<number>(Date.now());
const user_login_log = ref<UserLoginLog | null>(null);
const the_switch = ref(false);
const loadingBar = useLoadingBar();

const columns = [
  {
    title: "上线时间",
    key: "online_time",
    render(row: EveryLoginData) {
      return unix_format(row.online_time);
    },
  },
  {
    title: "下线时间",
    key: "offline_time",
    render(row: EveryLoginData) {
      return unix_format(row.offline_time);
    },
  },
  {
    title: "在线时长(分钟)",
    key: "used_duration",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.used_duration - row2.used_duration,
  },
  {
    title: "消耗流量(MB)",
    key: "used_flow",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.used_flow - row2.used_flow,
  },
  {
    title: "花费(元)",
    key: "cost",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.cost - row2.cost,
  },
  {
    title: "ipv4 ⬆(MB)",
    key: "ipv4_up",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.ipv4_up - row2.ipv4_up,
  },
  {
    title: "ipv4 ⬇(MB)",
    key: "ipv4_down",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.ipv4_down - row2.ipv4_down,
  },
  {
    title: "ipv6 ⬆(MB)",
    key: "ipv6_up",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.ipv6_up - row2.ipv6_up,
  },
  {
    title: "ipv6 ⬇(MB)",
    key: "ipv6_down",
    sorter: (row1: EveryLoginData, row2: EveryLoginData) =>
      row1.ipv6_down - row2.ipv6_down,
  },
  { title: "ipv4 地址", key: "ipv4_addr" },
  { title: "ipv6 地址", key: "ipv6_addr" },
];

onMounted(() => {
  get_user_login_log();
});

const get_user_login_log = async () => {
  loadingBar.start();
  if (the_switch.value === true) {
    let res = await invoke("load_user_login_log", {
      startDate: Math.floor(date_range.value[0] / 1000) + 8 * 3600,
      endDate: Math.floor(date_range.value[1] / 1000) + 8 * 3600,
    }).catch((err) => {
      pop_message.error(err);
      loadingBar.error();
    });
    // console.log(res as string);
    user_login_log.value = JSON.parse(res as string);
  } else {
    let res = await invoke("load_user_login_log", {
      startDate: Math.floor(a_date.value / 1000) + 8 * 3600,
      endDate: Math.floor(a_date.value / 1000) + 8 * 3600,
    }).catch((err) => {
      pop_message.error(err);
      loadingBar.error();
    });
    // console.log(res as string);
    user_login_log.value = JSON.parse(res as string);
  }
  loadingBar.finish();
};

const unix_format = (unix: number) => {
  return dayjs.unix(unix - 8 * 3600).format("YYYY-MM-DD HH:mm:ss");
};

const mb2gb = (mb: number | undefined) => {
  if (mb === undefined) return 0;
  else return parseFloat((mb / 1024).toFixed(2));
};
</script>

<template>
  <n-scrollbar x-scrollable style="max-height: 100vh">
    <div class="container">
      <n-h2 prefix="bar" type="success" style="margin-top: 15px">
        <n-text type="success"> 每日使用详情 </n-text>
      </n-h2>
      <n-switch
        v-model:value="the_switch"
        :rail-style="railStyle"
        style="margin-bottom: 10px"
      >
        <template #checked> 选很多天 </template>
        <template #unchecked> 选一天 </template>
      </n-switch>
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
      <div v-if="user_login_log !== null" class="show-data">
        <n-thing title="该段时间" content-style="margin-top: 10px;">
          <template #description>
            <n-table
              :bordered="false"
              :single-line="false"
              striped
              class="thistime"
            >
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
                  <td>{{ mb2gb(user_login_log?.ipv4_down) }} GB</td>
                  <td>{{ mb2gb(user_login_log?.ipv4_up) }} GB</td>
                  <td>{{ mb2gb(user_login_log?.ipv6_down) }} GB</td>
                  <td>{{ mb2gb(user_login_log?.ipv6_up) }} GB</td>
                </tr>
                <tr>
                  <td>💰 花费:</td>
                  <td>🕙 使用时长:</td>
                  <td>🛜 消耗流量:</td>
                  <td></td>
                </tr>
                <tr>
                  <td>{{ user_login_log?.cost.toFixed(2) }} 元</td>
                  <td>{{ min2hour(user_login_log?.used_duration) }} h</td>
                  <td>{{ user_login_log?.used_flow }} MB</td>
                  <td></td>
                </tr>
              </tbody>
            </n-table>
          </template>
        </n-thing>
        <br />
        <n-data-table
          :columns="columns"
          :data="user_login_log?.every_login_data"
          :bordered="false"
          :max-height="600"
        />
      </div>
      <br />
    </div>
  </n-scrollbar>
</template>

<style scoped>
.container {
  overflow: auto;
  padding: 10px;
}
.show-data {
  margin-top: 10px;
}
</style>
