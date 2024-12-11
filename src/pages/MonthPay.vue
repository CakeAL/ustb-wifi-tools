<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useLoadingBar, useMessage } from "naive-ui";
import { mb2gb, min2day, min2hour } from "../helper";
import YearlyChart from "../components/YearlyChart.vue";

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

const monthly_columns = [
  {
    title: "月份",
    key: "month",
    sorter: (row1: { month: number }, row2: { month: number }) =>
      row1.month - row2.month,
  },
  {
    title: "花费(元)",
    key: "month_cost",
    sorter: (row1: { month_cost: number }, row2: { month_cost: number }) =>
      row1.month_cost - row2.month_cost,
  },
  {
    title: "流量(MB)",
    key: "month_used_flow",
    sorter: (
      row1: { month_used_flow: number },
      row2: { month_used_flow: number }
    ) => row1.month_used_flow - row2.month_used_flow,
  },
  {
    title: "使用时长(分钟)",
    key: "month_used_duration",
    sorter: (
      row1: { month_used_duration: number },
      row2: { month_used_duration: number }
    ) => row1.month_used_duration - row2.month_used_duration,
  },
];

onMounted(() => {
  load_month_pay().then(() => handleUpdateValue(tabValue.value));
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
  handleUpdateValue(tabValue.value);
};

const chartData = ref<Array<number>>([]);
const tabValue = ref("flow");

const handleUpdateValue = (value: string) => {
  switch (value) {
    case "cost":
      chartData.value = month_pay?.value?.monthly_data.map(
        (v) => v.month_cost
      ) as Array<number>;
      return true;
    case "flow":
      chartData.value = month_pay?.value?.monthly_data.map((v) =>
        mb2gb(v.month_used_flow)
      ) as Array<number>;
      return true;
    case "duration":
      chartData.value = month_pay?.value?.monthly_data.map(
        (v) => v.month_used_duration
      ) as Array<number>;
      return true;
  }
};
</script>

<template>
  <n-scrollbar style="max-height: 100vh">
    <div class="container">
      <n-h2 prefix="bar" type="success" style="margin-top: 15px">
        <n-text type="success"> 年度扣费账单 </n-text>
      </n-h2>
      <n-select
        v-model:value="year"
        :options="year_options"
        @update:value="load_month_pay"
      />
      <div v-if="month_pay !== undefined" class="show-data">
        <n-card hoverable class="my-card">
          <n-grid x-gap="12" :cols="3">
            <n-gi>
              <n-statistic label="总共花费">
                {{ month_pay?.year_cost }} 元
              </n-statistic>
            </n-gi>
            <n-gi>
              <n-popover trigger="hover" placement="top-start">
                <template #trigger>
                  <n-statistic label="使用流量">
                    {{ mb2gb(month_pay?.year_used_flow) }} GB
                  </n-statistic>
                </template>
                {{ month_pay?.year_used_flow }} MB
              </n-popover>
            </n-gi>
            <n-gi>
              <n-popover trigger="hover" placement="top-start">
                <template #trigger>
                  <n-statistic label="使用时长">
                    {{ min2hour(month_pay?.year_used_duration) }} 小时
                  </n-statistic>
                </template>
                {{ month_pay?.year_used_duration }} 分钟，约合
                {{ min2hour(month_pay?.year_used_duration) }} 小时，{{
                  min2day(month_pay?.year_used_duration)
                }}
                天（不同设备使用时长会叠加）。
              </n-popover>
            </n-gi>
          </n-grid>
        </n-card>
        <n-tabs
          type="segment"
          animated
          @update:value="handleUpdateValue"
          v-model:value="tabValue"
        >
          <n-tab-pane name="flow" tab="流量(GB)" style="padding: 0"> </n-tab-pane>
          <n-tab-pane name="cost" tab="花费(元)" style="padding: 0"> </n-tab-pane>
          <n-tab-pane name="duration" tab="使用时长(分钟)" style="padding: 0"> </n-tab-pane>
        </n-tabs>
        <YearlyChart
          :month="month_pay?.monthly_data.map((v) => v.month)"
          :data="chartData"
          style="margin-top: 5px;"
        ></YearlyChart>
        <n-data-table
          :columns="monthly_columns"
          :data="month_pay?.monthly_data"
          style="margin-top: 12px"
        />
      </div></div
  ></n-scrollbar>
</template>

<style scoped>
.container {
  overflow: auto;
  padding: 10px;
}
.my-card {
  margin: 5px 0;
  width: 100%;
  background: rgba(255, 255, 255, 0.1);
}
</style>
