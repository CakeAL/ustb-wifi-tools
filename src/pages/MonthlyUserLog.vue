<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { EveryLoginData } from "./UserLoginLog.vue";
import { useMessage } from "naive-ui";

import dayjs from "dayjs";

const pop_message = useMessage();
const monthly_user_log = ref<Array<EveryLoginData>>([]);
const start_date = ref<number>(Date.now());

const get_monthly_user_log = async () => {
  let res = await invoke("load_monthly_login_log", {
    startDate: Math.floor(start_date.value / 1000) + 8 * 3600,
    days: dayjs.unix(start_date.value / 1000).daysInMonth(),
  }).catch((err) => pop_message.error(err));
  monthly_user_log.value = JSON.parse(res as string);

  console.log(monthly_user_log.value);
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
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
}
</style>
