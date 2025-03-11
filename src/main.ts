import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import { LineChart, LineSeriesOption } from "echarts/charts";
import {
  GridComponent,
  GridComponentOption,
  LegendComponent,
  LegendComponentOption,
  TitleComponent,
  TitleComponentOption,
  ToolboxComponent,
  ToolboxComponentOption,
  TooltipComponent,
  TooltipComponentOption,
} from "echarts/components";
import * as echarts from "echarts/core";
import { UniversalTransition } from "echarts/features";
import { CanvasRenderer } from "echarts/renderers";

echarts.use([
  TitleComponent,
  ToolboxComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  LineChart,
  CanvasRenderer,
  UniversalTransition,
]);

export type EChartsOption = echarts.ComposeOption<
  | TitleComponentOption
  | ToolboxComponentOption
  | TooltipComponentOption
  | GridComponentOption
  | LegendComponentOption
  | LineSeriesOption
>;

fetch("/macarons.json")
  .then((r) => r.json())
  .then((theme) => {
    echarts.registerTheme("macarons", theme);
  });

export { echarts };

createApp(App).mount("#app");
