<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted, CSSProperties } from "vue";
import { useMessage, useLoadingBar } from "naive-ui";

const loadingBar = useLoadingBar();
const pop_message = useMessage();
const sessionid = ref<string>("");
const user_name = ref<string>("");
const password = ref<string>("");
const button_disabled = ref<boolean>(false);
const login_state = ref<boolean>(false);
const login_via_vpn = ref<boolean>(false);

onMounted(() => {
  check_login_state();
  load_setting();
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

const get_cookies = async () => {
  if (user_name.value.length === 0 || password.value.length === 0) {
    pop_message.error("请先输入学号和密码");
    return;
  }
  loadingBar.start();
  button_disabled.value = true;
  let has_error = false;
  // 判断当前是否通过校园网 vpn 登陆
  let get_cookie_func = "get_cookie";
  if (login_via_vpn.value === true) {
    get_cookie_func = "get_cookie_vpn";
  }
  sessionid.value = (await invoke(get_cookie_func, {
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

const logout = async () => {
  let has_error = false;
  let res = await invoke("logout").catch((err) => {
    pop_message.error(err);
    has_error = true;
  });
  if (has_error === false) {
    pop_message.success(res as string);
  }
};

const set_setting = async () => {
  await invoke("set_setting").catch((err) => pop_message.error(err));
};

const railStyle = ({
  focused,
  checked,
}: {
  focused: boolean;
  checked: boolean;
}) => {
  const style: CSSProperties = {};
  if (checked) {
    style.background = "#4b9e5f";
    if (focused) {
      style.boxShadow = "0 0 0 2px #dbecdfff";
    }
  } else {
    style.background = "#2080f0";
    if (focused) {
      style.boxShadow = "0 0 0 2px #2080f040";
    }
  }
  return style;
};

const manually_check_update = () => {
  invoke("manually_check_update").catch((err) => pop_message.error(err));
};

const submit_login_ustb_wifi = () => {
  invoke("submit_login_ustb_wifi", {
    userName: user_name.value,
    password: password.value,
  })
    .then((res) => pop_message.success(res as string))
    .catch((err) => pop_message.error(err));
};
</script>

<template>
  <div class="container">
    <n-space vertical>
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
          style="margin-top: 5px"
        />
        <n-space>
          <n-switch
            v-model:value="login_via_vpn"
            :rail-style="railStyle"
            class="my-switch"
          >
            <template #checked> 我不在校园网 </template>
            <template #unchecked> 我在校园网 </template>
          </n-switch>
          <n-button
            strong
            secondary
            type="primary"
            @click="get_cookies"
            :disabled="button_disabled"
            class="my-button"
          >
            点我登陆校园网后台获取 cookie⭐️
          </n-button>
        </n-space>
        <h3 v-if="button_disabled === true">登录中...</h3>
      </div>
      <div v-else>
        <h3>您已登录，如果其他页面不能获取到信息，请关闭软件重新打开。</h3>
      </div>
      <!-- <h3>当前有效JSESSIONID：{{ sessionid }}</h3>
      <h4>
        ⬆️这个东西是当前打开应用期间的访问你的校园网数据的一个凭证，如果你发给其他人，那么一段时间内别人也可以看你的数据，这很危险，孩子。
      </h4> -->
      <n-button strong secondary type="info" @click="logout"> 登出 </n-button>
      <h4>如果想自己更改配置文件：</h4>
      <p>
        Windows: C:\Users\%UserName%\AppData\Roaming\ustb-wifi-tools\config.json
      </p>
      <p>macOS: ~/Library/Application Support/ustb-wifi-tools/config.json</p>
      <n-popover trigger="hover">
        <template #trigger>
          <n-button
            strong
            secondary
            type="primary"
            @click="submit_login_ustb_wifi"
            :disabled="button_disabled"
            class="my-button"
          >
            点我登陆校园网
          </n-button>
        </template>
        <span
          >当你被校园网登录“Radius认证超时！”搞烦了可以用，当然也可以直接用！</span
        >
      </n-popover>
      <p>用来手动检查更新的按钮：</p>
      <n-button tertiary type="info" @click="manually_check_update">
        我是用来手动检查更新的按钮
      </n-button>
    </n-space>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
  margin: 5px;
}
.my-switch {
  margin-top: 10px;
}
.my-button {
  margin-top: 5px;
}
</style>
