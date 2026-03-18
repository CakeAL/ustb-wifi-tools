<script setup lang="ts">
import { onMounted, watch } from "vue";
import { echarts, EChartsOption } from "../main";
import { UserOnlineLogRow } from "../pages/UserOnlineLog.vue";

const props = defineProps<{
  daily_log: Array<UserOnlineLogRow>;
}>();

const label = {
  formatter: function(params: any) {
    return Math.round(params.value).toString();
  },
};

onMounted(() => {
  if (props.daily_log) {
    renderChart(props.daily_log);
  }
});

watch(
  () => {
    props.daily_log;
  },
  () => {
    if (props.daily_log) {
      renderChart(props.daily_log);
    }
  },
  { deep: true },
);

function renderChart(daily_log: Array<UserOnlineLogRow>) {
  var end = 0;
  for (var i = daily_log.length; i > 0; i--) {
    if (
      daily_log[i - 1].flddownflowIPV4
          + daily_log[i - 1].fldupflowIPV4
          + daily_log[i - 1].flddownflowIPV6
          + daily_log[i - 1].fldupflowIPV6
        === 0
    ) {
      end++;
    } else {
      break;
    }
  }
  var daily_log = daily_log.slice(
    0,
    daily_log.length - end,
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
        data: Array.from({ length: daily_log.length }, (_, i) => {
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
        data: daily_log.map((v) => v.fldupflowIPV6),
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
        data: daily_log.map((v) => v.flddownflowIPV6),
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
        data: daily_log.map((v) => v.fldupflowIPV4),
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
        data: daily_log.map((v) => v.flddownflowIPV4),
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
