<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";
import { useMessage } from "naive-ui";
import { SearchOutline } from "@vicons/ionicons5";

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
const year = ref<number>(0);
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

onMounted(() => {
  //   load_month_pay();
});

const load_month_pay = async () => {
  if (year.value == 0) return;
  let res = await invoke("load_month_pay", { year: year.value }).catch((err) =>
    pop_message.error(err)
  );
  // console.log(res as string);
  month_pay.value = JSON.parse(res as string);
  console.log(month_pay.value);
};
</script>

<template>
  <div class="container">
    <h2>年度扣费账单</h2>
    <p>选择一个年份：</p>
    <n-select v-model:value="year" :options="year_options" />
    <n-button text @click="load_month_pay" style="font-size: 24px">
      <n-icon>
        <SearchOutline />
      </n-icon>
    </n-button>
  </div>
</template>

<style scoped></style>
