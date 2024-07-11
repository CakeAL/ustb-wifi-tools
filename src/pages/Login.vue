<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, onMounted } from "vue";
import { useMessage, useLoadingBar } from "naive-ui";

const loadingBar = useLoadingBar();
const pop_message = useMessage();
const has_browser = ref<boolean>(false);
const sessionid = ref<string>("");
const user_name = ref<string>("");
const password = ref<string>("");
const button_disabled = ref<boolean>(false);
const login_state = ref<boolean>(false);

onMounted(() => {
  check_login_state();
  load_setting()
    .then(check_has_browser)
    .then(() => {
      // 有浏览器并且没登录时候
      if (has_browser.value === true && login_state.value === false) {
        get_cookies();
      }
    });
});

const load_setting = async () => {
  let res = (await invoke("load_setting").catch((err) =>
    pop_message.error(err)
  )) as string;
  if (res.length > 0) {
    let settings = JSON.parse(res);
    user_name.value = settings.username;
    password.value = settings.password;
  }
};

const check_login_state = async () => {
  let res = (await invoke("get_jsessionid")) as string;
  if (res.length > 0) {
    sessionid.value = res;
    login_state.value = true;
  }
};

const check_has_browser = async () => {
  has_browser.value = (await invoke("check_has_browser").catch((err) =>
    pop_message.error(err)
  )) as boolean;
};

const set_browser_path = async () => {
  await invoke("set_browser_path").catch((err) => pop_message.error(err));
};

const get_cookies = async () => {
  if (user_name.value.length === 0 || password.value.length === 0) {
    pop_message.error("请先输入学号和密码");
    return;
  }
  loadingBar.start();
  button_disabled.value = true;
  let has_error = false;
  sessionid.value = (await invoke("get_cookie", {
    userName: user_name.value,
    password: password.value,
  })
    .catch((err) => {
      pop_message.error(err);
      loadingBar.error();
      has_error = true;
      // 登录失败
      login_state.value = false;
    })
    .finally(() => {
      button_disabled.value = false;
      if (has_error === false) {
        // 登录成功
        loadingBar.finish();
        login_state.value = true;
        set_setting();
      } else {
        // 失败
        login_state.value = false;
      }
    })) as string;
};

const set_setting = async () => {
  await invoke("set_setting").catch((err) => pop_message.error(err));
};
</script>

<template>
  <div class="container">
    <div v-if="!has_browser">
      <h3>
        非常不幸，您的电脑上貌似没有
        <b>Edge/Chrome</b> 浏览器，或许它们只是被安装到其他地方了？
      </h3>
      <h3>或许您可以手动选择浏览器可执行文件位置？</h3>
      <h4>
        Windows 上：请选择后缀名为 <b>.exe</b> 的可执行文件（不是快捷方式！）。
      </h4>
      <h4>macOS 上：请选择“应用程序”文件夹中的 APP</h4>
      <n-button strong secondary type="primary" @click="set_browser_path">
        点我来选择浏览器可执行文件
      </n-button>
    </div>
    <n-space vertical v-else>
      <div v-if="!login_state">
        <h3>
          在下方输入学号和密码（如果没改过，是身份证后8位，数据均在本地存储）：
        </h3>
        <n-input
          v-model:value="user_name"
          type="text"
          placeholder="学号/工号"
          round
        />
        <n-input
          v-model:value="password"
          type="password"
          show-password-on="mousedown"
          placeholder="密码"
          round
        />
        <n-button
          strong
          secondary
          type="primary"
          @click="get_cookies"
          :disabled="button_disabled"
        >
          点我登陆获取 cookie⭐️
        </n-button>
        <h3 v-if="button_disabled === true">登录中...</h3>
      </div>
      <div v-else>
        <h3>您已登录，如果其他页面不能获取到信息，请关闭软件重新打开。</h3>
      </div>
      <h3>当前有效JSESSIONID：{{ sessionid }}</h3>
      <h4>
        ⬆️这个东西是当前打开应用期间的访问你的校园网数据的一个凭证，如果你发给其他人，那么一段时间内别人也可以看你的数据，这很危险，孩子。
      </h4>
    </n-space>
  </div>
</template>

<style scoped></style>
