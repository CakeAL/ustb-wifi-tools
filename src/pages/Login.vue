<script setup lang="ts">
import { ref, onMounted } from "vue";

const login_url = ref("/login");
const login_html = ref("");

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
</script>

<template>
  <div class="container">
    <h1>Login</h1>
    <!-- <div v-html="login_html"></div> -->
    <iframe  name = "iframeMap" id="iframeMapViewComponent"  v-bind:src="login_url"
               width="100%" height="100%"
               frameborder="0" scrolling="yes" ref="iframeDom"
      ></iframe>
  </div>
</template>

<style scoped>
#iframeMapViewComponent {
    height: 90vh;
    width: 100%;
}
</style>
