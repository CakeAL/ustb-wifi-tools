<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useMessage } from "naive-ui";
import dayjs from "dayjs";

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

interface EveryLoginData {
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
const user_login_log = ref<UserLoginLog | null>(null);

const get_user_login_log = async () => {
  let res = await invoke("load_user_login_log", {
    startDate: Math.floor(date_range.value[0] / 1000),
    endDate: Math.floor(date_range.value[1] / 1000),
  }).catch((err) => pop_message.error(err));
  // console.log(res as string);
  user_login_log.value = JSON.parse(res as string);
  if (user_login_log.value !== null) {
  }
};

const unix_format = (unix: number) => {
  return dayjs.unix(unix - 8 * 3600).format("YYYY-MM-DD HH:mm:ss");
};
</script>

<template>
  <div class="container">
    <h2>每日使用详情</h2>
    <p>选择一个时间段：</p>
    <n-date-picker
      v-model:value="date_range"
      type="daterange"
      clearable
      @update:value="get_user_login_log"
    />
    <div v-if="user_login_log !== null" class="show-data">
      <p>该段时间：</p>
      <p>
        ipv4 上行：{{ user_login_log?.ipv4_up }} MB；下行：{{
          user_login_log?.ipv4_down
        }}
        MB。
      </p>
      <p>
        ipv6 上行：{{ user_login_log?.ipv6_up }} MB；下行：{{
          user_login_log?.ipv6_down
        }}
        MB。
      </p>
      <p>消耗校园网流量：{{ user_login_log?.used_flow }} MB。</p>
      <p>花费金额：{{ user_login_log?.cost }} 元。</p>
      <p>使用时长: {{ user_login_log?.used_duration }} 分钟。</p>

      <n-table :bordered="false" :single-line="false">
        <thead>
          <tr>
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
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
}
</style>
