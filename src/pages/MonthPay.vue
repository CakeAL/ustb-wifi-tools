<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useLoadingBar, useMessage } from "naive-ui";
import { mb2gb } from "../helper";

interface MonthlyData {
  month: number;
  month_cost: number;
  month_used_flow: number;
  month_used_duration: number;
}

interface YearlyData {
  year_cost: number;
  year_used_duration: number;
  year_used_flow: number;
  monthly_data: MonthlyData[];
}

const pop_message = useMessage();
const month_pay = ref<YearlyData | null>(null);
const year = ref<number>(new Date().getFullYear());
const year_options = Array.from(
  { length: new Date().getFullYear() - 2015 + 1 },
  (_, i) => {
    const year = new Date().getFullYear() - i;
    return {
      label: String(year),
      value: year,
    };
  }
);
const loadingBar = useLoadingBar();

const month_column = {
  title: "月份",
  key: "month",
};

const month_cost_column = {
  title: "花费(元)",
  key: "month_cost",
};

const month_used_flow_column = {
  title: "流量(MB)",
  key: "month_used_flow",
};

const month_used_duration_column = {
  title: "使用时长(分钟)",
  key: "month_used_duration",
};

const monthly_columns = [
  month_column,
  month_cost_column,
  month_used_flow_column,
  month_used_duration_column,
];

onMounted(() => {
  load_month_pay();
});

const load_month_pay = async () => {
  loadingBar.start();
  if (year.value == 0) return;
  let res = await invoke("load_month_pay", { year: year.value }).catch(
    (err) => {
      pop_message.error(err);
      loadingBar.error();
    }
  );
  // console.log(res as string);
  month_pay.value = JSON.parse(res as string);
  // console.log(month_pay.value);
  loadingBar.finish();
};

const min2hour = (min: number | undefined) => {
  return parseFloat(((min as number) / 60).toFixed(2));
};

const min2day = (min: number | undefined) => {
  return parseFloat(((min as number) / 60 / 24).toFixed(2));
};

</script>

<template>
  <div class="container">
    <n-scrollbar style="max-height: 100vh">
      <n-h2 prefix="bar" type="success" style="margin-top: 15px">
        <n-text type="success"> 年度扣费账单 </n-text>
      </n-h2>
      <p>选择一个年份：</p>
      <n-select
        v-model:value="year"
        :options="year_options"
        @update:value="load_month_pay" />
      <div v-if="month_pay !== undefined" class="show-data">
        <p>这一年一共花费 {{ month_pay?.year_cost }} 元。</p>
        <p>
          总共使用时长 {{ month_pay?.year_used_duration }} 分钟，约合
          {{ min2hour(month_pay?.year_used_duration) }} 小时，{{
            min2day(month_pay?.year_used_duration)
          }}
          天（不同设备使用时长会叠加）。
        </p>
        <p>
          总共使用流量 {{ month_pay?.year_used_flow }} MB，约合
          {{ mb2gb(month_pay?.year_used_flow) }} GB。
        </p>
        <n-data-table
          :columns="monthly_columns"
          :data="month_pay?.monthly_data"
        /></div
    ></n-scrollbar>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
  margin: 5px;
}
</style>
