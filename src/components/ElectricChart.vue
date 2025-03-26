<script setup lang="ts">
import { onMounted, watch } from "vue";
import { echarts, EChartsOption } from "../main";
import { RemainingElectricity } from "../pages/ElectricBill.vue";
const props = defineProps<{
  data: Array<RemainingElectricity>;
}>();

const label = {
  formatter: function(params: any) {
    return Math.round(params.value).toString();
  },
};

onMounted(() => {
  if (props.data) {
    renderChart(props.data);
  }
});

watch(
  () => {
    props.data;
  },
  () => {
    if (props.data) {
      renderChart(props.data);
    }
  },
  { deep: true },
);

function renderChart(data: Array<RemainingElectricity>) {
  var chartDom = document.getElementById("chart")!;
  var myChart = echarts.init(chartDom, "macarons");
  let dateList = data.map((v) => v.date * 1000);
  let valueList = data.map((v) => v.remain);
  let maxValue = Math.max(...valueList);
  var option: EChartsOption = {
    visualMap: [
      {
        show: false,
        type: "continuous",
        seriesIndex: 0,
        min: -10,
        max: maxValue,
        inRange: {
          color: ["#eccc68", "#1abc9c", "#27ae60"],
        },
      },
    ],
    tooltip: {
      trigger: "axis",
      axisPointer: {
        type: "cross",
        label: {
          backgroundColor: "#6a7985",
        },
      },
    },
    xAxis: {
      type: "time", // 时间轴，自动适配不均匀的时间间隔
      axisLabel: {
        formatter: "{yyyy}-{MM}-{dd}", // 格式化时间显示
      },
    },
    yAxis: {},
    grid: {
      left: "2%",
      right: "5%",
      bottom: "3%",
      top: "3%",
      containLabel: true,
    },
    series: [
      {
        data: data.map((v) => [v.date * 1000, v.remain]),
        type: "line",
        label: label,
      },
    ],
  };

  myChart.setOption(option);

  window.addEventListener("resize", function() {
    myChart.resize();
  });
}
</script>

<template>
  <div id="chart"></div>
</template>

<style scoped>
#chart {
  height: 48vh;
  width: 100%;
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  padding: 10px 0;
  margin-top: 10px;
}
</style>
