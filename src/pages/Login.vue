<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { useMessage } from "naive-ui";

// const login_url = ref("/nav_login");
// const login_html = ref("");
const pop_message = useMessage();
const sessionid = ref<string>("");

const get_cookies = async () => {
  sessionid.value = await invoke("get_cookie", {userName: "todo", password: "todo"}).catch((err) => pop_message.error(err)) as string;
};

const open_nav_login = async () => {
  await invoke("open_nav_login").catch((err) => pop_message.error(err));
};
</script>

<template>
  <div class="container">
    <h2>首先点击下方按钮，弹出校园网登录页</h2>
    <n-button strong secondary type="primary" @click="open_nav_login">
      点我打开登录页
    </n-button>
    <h2>然后在弹出的窗口登录自助服务系统</h2>
    <h2>确保已经登录成功，点击下方按钮确认</h2>
    <n-button strong secondary type="primary" @click="get_cookies">
      登录成功后点我
    </n-button>
    <h3>当前有效JSESSIONID（请不要截图给他人）：{{ sessionid }}</h3>
  </div>
</template>

<style scoped>
</style>
