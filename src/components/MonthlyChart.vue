<script setup lang="ts">
import { onMounted, watch } from "vue";
import { echarts, EChartsOption } from "../main";
import { EveryLoginData } from "../pages/UserLoginLog.vue";

const props = defineProps({
  monthly_user_log: Array<EveryLoginData>,
});

const label = {
  formatter: function(params: any) {
    return Math.round(params.value).toString();
  },
};

onMounted(() => {
  if (props.monthly_user_log) {
    renderChart(props.monthly_user_log);
  }
});

watch(
  () => {
    props.monthly_user_log;
  },
  () => {
    if (props.monthly_user_log) {
      renderChart(props.monthly_user_log);
    }
  },
  { deep: true },
);

function renderChart(monthly_user_log: Array<EveryLoginData>) {
  var end = 0;
  for (var i = monthly_user_log.length; i > 0; i--) {
    if (
      monthly_user_log[i - 1].ipv4_down
          + monthly_user_log[i - 1].ipv4_up
          + monthly_user_log[i - 1].ipv6_down
          + monthly_user_log[i - 1].ipv6_up
        === 0
    ) {
      end++;
    } else {
      break;
    }
  }
  var monthly_user_log = monthly_user_log.slice(
    0,
    monthly_user_log.length - end,
  );
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
    legend: {
      data: ["ipv6 ⬆", "ipv6 ⬇", "ipv4 ⬆", "ipv4 ⬇"],
    },
    grid: {
      left: "2%",
      right: "5%",
      bottom: "3%",
      containLabel: true,
    },
    xAxis: [
      {
        type: "category",
        boundaryGap: false,
        data: Array.from({ length: monthly_user_log.length }, (_, i) => {
          i = i + 1;
          return i.toString() + "日";
        }),
      },
    ],
    yAxis: [
      {
        type: "value",
        name: "MB",
      },
    ],
    series: [
      {
        name: "ipv6 ⬆",
        type: "line",
        stack: "Total",
        areaStyle: {},
        emphasis: {
          focus: "series",
          label: {
            show: true,
            formatter: function(params) {
              let value = params.value as number;
              return value.toFixed(0) + " MB";
            },
          },
        },
        data: monthly_user_log.map((v) => v.ipv6_up),
        label: label,
        tooltip: {
          valueFormatter: (value) => Math.round(value as number) + " MB",
        },
      },
      {
        name: "ipv6 ⬇",
        type: "line",
        stack: "Total",
        areaStyle: {},
        emphasis: {
          focus: "series",
        },
        data: monthly_user_log.map((v) => v.ipv6_down),
        label: label,
        tooltip: {
          valueFormatter: (value) => Math.round(value as number) + " MB",
        },
      },
      {
        name: "ipv4 ⬆",
        type: "line",
        stack: "Total",
        areaStyle: {},
        emphasis: {
          focus: "series",
        },
        data: monthly_user_log.map((v) => v.ipv4_up),
        label: label,
        tooltip: {
          valueFormatter: (value) => Math.round(value as number) + " MB",
        },
      },
      {
        name: "ipv4 ⬇",
        type: "line",
        stack: "Total",
        label: {
          show: true,
          position: "top",
          formatter: function(params: any) {
            return Math.round(params.value).toString();
          },
        },
        areaStyle: {},
        emphasis: {
          focus: "series",
        },
        data: monthly_user_log.map((v) => v.ipv4_down),
        tooltip: {
          valueFormatter: (value) => Math.round(value as number) + " MB",
        },
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
  height: 65vh;
  width: 100%;
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  padding: 10px 0;
}
</style>
