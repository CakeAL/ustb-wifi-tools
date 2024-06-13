<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, onMounted } from "vue";
import { useMessage } from "naive-ui";

// SameSite未解决
const login_url = ref("/nav_login");
// const login_html = ref("");
const pop_message = useMessage();

onMounted(() => {
  // load_login_page();
});

// const load_login_page = async () => {
//     try {
//         http.fetch("http://202.204.60.7:8080/nav_login", {
//           method: "GET",
//           headers: {
//             ResponseType: "text",
//           },
//         }).then((response) => {
//           // console.log(response);
//         });
//     } catch (e) {
//         login_html.value = "获取登陆页面失败，可能是没连接校园网？"
//         console.error(e);
//     }
// };

const get_cookies = async () => {
  invoke("get_cookie").catch((err) => pop_message.error(err));
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
