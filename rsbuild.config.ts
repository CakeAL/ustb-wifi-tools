import { defineConfig, loadEnv } from "@rsbuild/core";
import { pluginVue } from "@rsbuild/plugin-vue";
import AutoImport from "unplugin-auto-import/rspack";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/rspack";

const { publicVars } = loadEnv({ prefixes: ["VITE_"] });

export default defineConfig({
  plugins: [
    pluginVue(),
  ],
  tools: {
    rspack: {
      plugins: [
        AutoImport({
          imports: [
            "vue",
            {
              "naive-ui": [
                "useDialog",
                "useMessage",
                "useNotification",
                "useLoadingBar",
              ],
            },
          ],
        }),
        Components({
          resolvers: [NaiveUiResolver()],
        }),
      ],
    },
  },
  performance: {
    chunkSplit: {
      strategy: "split-by-experience",
    },
    preload: {
      type: "all-chunks",
    },
  },
  source: {
    define: publicVars,
    entry: {
      index: "./src/main.ts",
    },
  },
  dev: {
    watchFiles: {
      paths: "!src-tauri/**",
    },
  },
  server: {
    port: 1420,
    strictPort: true,
  },
});
