<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import dayjs from "dayjs";
import { useLoadingBar, useMessage } from "naive-ui";
import { onMounted, ref } from "vue";
import YearlyChart from "../components/YearlyChart.vue";
import { mb2gb, min2day, min2hour } from "../helper";

interface MonthPayData {
  summary: Summary;
  total: number;
  rows: RowArray[];
}

interface Summary {
  USETIME: number;
  USEBASEMONEY: number;
  USEFLOW: number;
  USEDMONEY: number;
}

type RowArray = [
  number, // startTime (1769875200000)
  number, // endTime (1772294400000)
  string, // userType ("")
  number, // baseMoney (0.0)
  number, // usedMoney (0.0)
  number, // usedTime (3615.0)
  number, // usedFlow (22505.904)
  number, // updateTime (1772294786000)
];

const pop_message = useMessage();
const month_pay = ref<MonthPayData | null>(null);
const year = ref<number>(
  new Date().getMonth() > 0
    ? new Date().getFullYear()
    : new Date().getFullYear() - 1,
);
const year_options = Array.from(
  { length: new Date().getFullYear() - 2015 + 1 },
  (_, i) => {
    const year = new Date().getFullYear() - i;
    return {
      label: String(year),
      value: year,
    };
  },
);
const loadingBar = useLoadingBar();

const monthly_columns = [
  {
    title: "月份",
    key: "month",
    render: (row: RowArray) => dayjs(row[0]).format("YY-MM"),
    sorter: (row1: RowArray, row2: RowArray) => row1[0] - row2[0],
  },
  {
    title: "花费(元)",
    key: "month_cost",
    render: (row: RowArray) => row[4], // usedMoney is at index 4
    sorter: (row1: RowArray, row2: RowArray) => row1[4] - row2[4],
  },
  {
    title: "流量(MB)",
    key: "month_used_flow",
    render: (row: RowArray) => row[6], // usedFlow is at index 6
    sorter: (row1: RowArray, row2: RowArray) => row1[6] - row2[6],
  },
  {
    title: "使用时长(分钟)",
    key: "month_used_duration",
    render: (row: RowArray) => row[5], // usedTime is at index 5
    sorter: (row1: RowArray, row2: RowArray) => row1[5] - row2[5],
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
    },
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
      chartData.value = month_pay?.value?.rows.map(
        (v) => v[4], // usedMoney is at index 4
      ) as Array<number>;
      return true;
    case "flow":
      chartData.value = month_pay?.value?.rows.map((v) =>
        parseFloat((v[6] / 1024).toFixed(2))
      ) as Array<number>;
      return true;
    case "duration":
      chartData.value = month_pay?.value?.rows.map(
        (v) => v[5], // usedTime is at index 5
      ) as Array<number>;
      return true;
  }
};
</script>

<template>
  <div>
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
              {{ month_pay?.summary.USEDMONEY }} 元
            </n-statistic>
          </n-gi>
          <n-gi>
            <n-popover trigger="hover" placement="top-start">
              <template #trigger>
                <n-statistic label="使用流量">
                  {{ mb2gb(month_pay?.summary.USEFLOW) }}
                </n-statistic>
              </template>
              {{ month_pay?.summary.USEFLOW }} MB
            </n-popover>
          </n-gi>
          <n-gi>
            <n-popover trigger="hover" placement="top-start">
              <template #trigger>
                <n-statistic label="使用时长">
                  {{
                    min2hour(
                      month_pay?.summary.USETIME,
                    )
                  }} 小时
                </n-statistic>
              </template>
              {{ month_pay?.summary.USETIME }} 分钟，约合
              {{ min2hour(month_pay?.summary.USETIME) }} 小时，{{
                min2day(month_pay?.summary.USETIME)
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
        <n-tab-pane name="duration" tab="使用时长(分钟)" style="padding: 0">
        </n-tab-pane>
      </n-tabs>
      <YearlyChart
        :month="month_pay?.rows.map((v) => dayjs(v[0]).format('YY-MM')) ?? []"
        :data="chartData"
        style="margin-top: 5px"
      >
      </YearlyChart>
      <n-data-table
        :columns="monthly_columns"
        :data="month_pay?.rows"
        style="margin-top: 12px"
      />
    </div>
  </div>
</template>

<style scoped></style>
