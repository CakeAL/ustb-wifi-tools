<script setup lang="ts">
import { onMounted, watch } from "vue";
import { echarts, EChartsOption } from "../main";

const props = defineProps({
  month: Array<number>,
  data: Array<number>,
});

const label = {
  formatter: function (params: any) {
    return Math.round(params.value).toString();
  },
};

onMounted(() => {
  if (props.data && props.month) {
    let data = props.data.reverse();
    let month = props.month.reverse();
    renderChart(month, data);
  }
});

watch(
  () => {
    props.data;
  },
  () => {
    if (props.data && props.month) {
      let data = props.data.reverse();
      let month = props.month.reverse();
      renderChart(month, data);
    }
  },
  { deep: true }
);

function renderChart(month: Array<number>, data: Array<number>) {
  var chartDom = document.getElementById("chart")!;
  var myChart = echarts.init(chartDom, "macarons");
  var option: EChartsOption = {
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
      type: "category",
      boundaryGap: false,
      data: month.map((v) => v.toString() + "月"),
    },
    yAxis: {
      type: "value",
    },
    grid: {
      left: "2%",
      right: "5%",
      bottom: "3%",
      top: "3%",
      containLabel: true,
    },
    series: [
      {
        data: data,
        type: "line",
        areaStyle: {},
        label: label,
      },
    ],
  };

  myChart.setOption(option);

  window.addEventListener("resize", function () {
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
}
</style>
