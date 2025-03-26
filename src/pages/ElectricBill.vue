<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useLoadingBar, useMessage } from "naive-ui";
import { computed, onMounted, ref } from "vue";
import ElectricChart from "../components/ElectricChart.vue";
import { date_format } from "../helper";

const ammeter_number = ref<string>("");
const remaining_electricity = ref<Array<RemainingElectricity>>([]);
const pop_message = useMessage();
const loadingBar = useLoadingBar();
const ammeter_data = ref(0);

const columns = [
  {
    title: "日期",
    key: "date",
    render(row: RemainingElectricity) {
      return date_format(row.date);
    },
  },
  {
    title: "剩余电量（度）",
    key: "remain",
    sorter: (row1: { remain: number }, row2: { remain: number }) =>
      row1.remain - row2.remain,
  },
  {
    title: "自从上次获取后平均每天使用量",
    key: "average",
    sorter: (row1: { average: number }, row2: { average: number }) =>
      row1.average - row2.average,
  },
];

onMounted(() => {
  load_ammeter_number().then(() => {
    if (ammeter_number.value !== "") {
      load_electric_bill();
    }
  });
});

export interface RemainingElectricity {
  date: number;
  remain: number;
  average: number;
}

const load_ammeter_number = async () => {
  let res =
    (await invoke("load_setting").catch((err) =>
      pop_message.error(err)
    )) as string;
  if (res.length > 0) {
    let settings = JSON.parse(res);
    ammeter_number.value = settings.ammeter_number;
  }
};

const load_electric_bill = async () => {
  loadingBar.start();
  let res = (await invoke("load_electric_bill")
    .catch((err) => pop_message.error(err))) as string;
  console.log(res);

  if (res.length > 0) {
    console.log(res);
    let parsed: [Array<RemainingElectricity>, String] = JSON.parse(res);
    remaining_electricity.value = parsed[0];
    if (parsed[1].length > 0) {
      pop_message.info(`${parsed[1]}`);
    }
  }
  loadingBar.finish();
};

const last_remain = computed(() => {
  let len = remaining_electricity.value.length;
  if (len >= 1) {
    return remaining_electricity.value[len - 1].remain;
  } else {
    return -NaN;
  }
});

const load_ammeter = async () => {
  let number = parseInt(ammeter_number.value);
  if (!isNaN(number)) {
    loadingBar.start();
    let res = await invoke("load_ammeter", {
      ammeterNumber: number,
    }).catch((err) => pop_message.error(err));
    loadingBar.finish();
    ammeter_data.value = res as number;
  } else {
    pop_message.error("电表号应该是纯数字!");
  }
};
</script>

<template>
  <div>
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 电费统计 </n-text>
    </n-h2>
    <n-card hoverable class="my-card">
      <n-statistic :label="ammeter_number + ' 目前剩余电量'">
        {{ last_remain }} kWh
      </n-statistic>
    </n-card>
    <ElectricChart :data="remaining_electricity"></ElectricChart>
    <n-data-table
      :columns="columns"
      :data="[...remaining_electricity].reverse()"
      style="margin-top: 12px"
    />
    <n-card hoverable class="my-card" title="说明">
      <n-p
      >本页面（加载时）可以自动获取已存储电表号的电表数据，并绘制成表格和曲线。</n-p>
      <n-p>学校电表数据后台更新不及时，部分天数可能一样是正常的</n-p>
      <n-p>如果没有电表号，在这里可以存一下：</n-p>
      <n-input
        v-model:value="ammeter_number"
        type="text"
        placeholder="电表号"
        @blur="load_ammeter"
        round
      />
      <n-p>当前剩余电量: {{ ammeter_data }} kWh </n-p>
    </n-card>
  </div>
</template>

<style scoped></style>
