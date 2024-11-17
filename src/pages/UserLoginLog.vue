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
  <div class="container">
    <n-scrollbar x-scrollable style="max-height: 100vh">
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
        <p>该段时间：</p>
        <n-table :bordered="false" :single-line="false" striped>
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
              <td>💰花费:</td>
              <td>🕙使用时长:</td>
              <td>🛜消耗流量:</td>
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
        <br />
        <div class="my-table">
          <n-table :bordered="false" :single-line="false">
            <thead>
              <tr class="first-line">
                <th>上线时间</th>
                <th>下线时间</th>
                <th>在线时长(分钟)</th>
                <th>消耗流量(MB)</th>
                <th>花费(元)</th>
                <th>ipv4 上行(MB)</th>
                <th>ipv4 下行(MB)</th>
                <th>ipv6 上行(MB)</th>
                <th>ipv6 下行(MB)</th>
                <th>ipv4 地址</th>
                <th>ipv6 地址</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="(log_info, index) in user_login_log?.every_login_data"
                :key="index"
              >
                <th>{{ unix_format(log_info.online_time) }}</th>
                <th>{{ unix_format(log_info.offline_time) }}</th>
                <th>{{ log_info.used_duration }}</th>
                <th>{{ log_info.used_flow }}</th>
                <th>{{ log_info.cost }}</th>
                <th>{{ log_info.ipv4_up }}</th>
                <th>{{ log_info.ipv4_down }}</th>
                <th>{{ log_info.ipv6_up }}</th>
                <th>{{ log_info.ipv6_down }}</th>
                <th>{{ log_info.ipv4_addr }}</th>
                <th>{{ log_info.ipv6_addr }}</th>
              </tr>
            </tbody>
          </n-table>
        </div>
      </div>
      <br />
    </n-scrollbar>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
  margin: 5px;
}
.my-table {
  width: calc(100vw - 215px);
  height: 80vh;
  overflow: auto;
}
.first-line > th {
  position: sticky;
  top: 0;
  z-index: 1; /* 确保表头在上方 */
}
</style>
