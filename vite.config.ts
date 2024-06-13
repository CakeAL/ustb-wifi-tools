import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
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

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    proxy: {
      '/nav_login': {
        target: 'http://202.204.60.7:8080/nav_login',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/nav_login/, '')
      },
      '/LoginAction.action': {
        target: 'http://202.204.60.7:8080/LoginAction.action',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/LoginAction.action/, '')
      },
      '/RandomCodeAction.action': {
        target: 'http://202.204.60.7:8080/RandomCodeAction.action',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/RandomCodeAction.action/, '')
      },
      '/js': {
        target: 'http://202.204.60.7:8080/js',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/js/, '')
      },
      '/style': {
        target: 'http://202.204.60.7:8080/style',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/style/, '')
      }
    },
  },
}));
