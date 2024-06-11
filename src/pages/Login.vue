<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, onMounted } from "vue";
import { useMessage } from "naive-ui";

const login_url = ref("/login");
// const login_html = ref("");
const pop_message = useMessage();

onMounted(() => {
  // load_login_page();
});

// const load_login_page = async () => {
//     try {
//         const response = await fetch(login_url.value);
//         if (!response.ok) {
//             throw new Error(`HTTP error! status: ${response.status}`);
//         }
//         login_html.value = await response.text();
//         // console.log(login_html.value);
//     } catch (e) {
//         login_html.value = "获取登陆页面失败，可能是没连接校园网？"
//         console.error(e);
//     }
// };

const get_cookies = async () => {
  const res = invoke("get_cookie").catch((err) => pop_message.error(err));
  console.log(res);
};
</script>

<template>
  <div class="container">
    <h1>Login</h1>
    <n-button strong secondary type="primary" @click="get_cookies">
      Click Me To Set Cookies
    </n-button>
    <!-- <div v-html="login_html"></div> -->
    <iframe
      name="iframeMap"
      id="iframeMapViewComponent"
      v-bind:src="login_url"
      width="100%"
      height="100%"
      frameborder="0"
      scrolling="yes"
      ref="iframeDom"
    ></iframe>
  </div>
</template>

<style scoped>
#iframeMapViewComponent {
  height: 90vh;
  width: 100%;
}
</style>
