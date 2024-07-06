<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { useMessage } from "naive-ui";

// const login_url = ref("/nav_login");
// const login_html = ref("");
const pop_message = useMessage();
const sessionid = ref<string>("");
const user_name = ref<string>("");
const password = ref<string>("");

const get_cookies = async () => {
  if (user_name.value.length === 0 || password.value.length === 0) {
    pop_message.error("请先输入学号和密码");
    return;
  }
  sessionid.value = (await invoke("get_cookie", {
    userName: user_name.value,
    password: password.value,
  }).catch((err) => pop_message.error(err))) as string;
};
</script>

<template>
  <div class="container">
    <h3>
      在下方输入学号和密码（如果没改过，是身份证后8位，数据均在本地存储）：
    </h3>
    <n-input v-model:value="user_name" type="text" placeholder="学号/工号" />
    <n-input
      v-model:value="password"
      type="password"
      show-password-on="mousedown"
      placeholder="密码"
    />
    <n-button strong secondary type="primary" @click="get_cookies">
      点我登陆获取 cookie⭐️
    </n-button>
    <h3>当前有效JSESSIONID：{{ sessionid }}</h3>
    <h4>⬆️这个东西是当前打开应用期间的访问你的校园网数据的一个凭证，如果你发给其他人，并且没有关闭当前 APP，那么别人也可以看你的数据，这很危险，孩子。</h4>
  </div>
</template>

<style scoped></style>
