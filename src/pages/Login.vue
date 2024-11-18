<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";
import { useMessage, useLoadingBar } from "naive-ui";
import { ColorPaletteOutline } from "@vicons/ionicons5";
import { open } from "@tauri-apps/plugin-shell";
import { dataDir } from "@tauri-apps/api/path";
import { check_update } from "../update";
import { railStyle } from "../helper";

const loadingBar = useLoadingBar();
const pop_message = useMessage();
const sessionid = ref<string>("");
const user_name = ref<string>("");
const password = ref<string>("");
const button_disabled = ref<boolean>(false);
const login_state = ref<boolean>(false);
const login_via_vpn = ref<boolean>(false);
const showModal = ref<boolean>(false);
const transparence = ref<number>(0);
const blur = ref<number>(0);

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
    transparence.value = settings.background_transparence;
    blur.value = settings.background_blur;
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

const manually_check_update = async () => {
  await check_update(true);
};

const submit_login_ustb_wifi = async () => {
  await invoke("submit_login_ustb_wifi", {
    userName: user_name.value,
    password: password.value,
  })
    .then((res) => pop_message.success(res as string))
    .catch((err) => pop_message.error(err));
};

const set_background_image = async () => {
  await invoke("set_background_image").catch((err) => pop_message.error(err));
};

const reset_background_image = async () => {
  await invoke("reset_background_image").catch((err) => pop_message.error(err));
};

const set_background_transparence = async () => {
  await invoke("set_background_transparence", {
    transparence: transparence.value,
  }).catch((err) => pop_message.error(err));
};

const set_background_blur = async () => {
  await invoke("set_background_blur", {
    blur: blur.value,
  }).catch((err) => pop_message.error(err));
};

const open_config = async () => {
  // explorer 还是强的，能识别斜杠
  let path = (await dataDir()) + "/ustb-wifi-tools";
  // console.log(path);
  await open(path);
};

const open_changepassword = async () => {
  await open("http://202.204.60.7:8080/nav_changePsw");
};

const open_guide = async () => {
  await open("https://mp.weixin.qq.com/s/1zavoaNZeqo7fNb2I_53Iw");
};
</script>

<template>
  <n-scrollbar style="max-height: 100vh">
    <div class="container">
      <n-space vertical>
        <div v-if="!login_state">
          <n-h3 prefix="bar" type="success" style="margin-top: 15px">
            在下方输入校园网学号和密码（数据均在本地存储）：
          </n-h3>
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
            <n-switch
              v-model:value="login_via_vpn"
              :rail-style="railStyle"
              class="my-switch"
            >
              <template #checked> 我不在校园网 </template>
              <template #unchecked> 我在校园网 </template>
            </n-switch>
          </n-space>
          <n-h3
            prefix="bar"
            type="success"
            style="margin-top: 15px"
            v-if="button_disabled === true"
            >登录中...</n-h3
          >
        </div>
        <div v-else>
          <n-h3 prefix="bar" type="success" style="margin-top: 15px"
            >您已登录，如果其他页面不能获取到信息，请点击登出再重新登录。</n-h3
          >
        </div>
        <n-grid :x-gap="12" :y-gap="8" :cols="2" style="margin-top: 20px">
          <n-grid-item>
            <n-card title="登出" hoverable @click="logout" class="my-card">
              想换个账号登录？
            </n-card> </n-grid-item
          ><n-grid-item>
            <n-card
              title="手动检查更新"
              hoverable
              @click="manually_check_update"
              class="my-card"
            >
              手动地检查更新。
            </n-card> </n-grid-item
          ><n-grid-item>
            <n-card
              title="登陆校园网"
              hoverable
              @click="submit_login_ustb_wifi"
              class="my-card"
            >
              当你被校园网登录“Radius认证超时！”搞烦了可以用，当然也可以直接用！
            </n-card>
          </n-grid-item>
          <n-grid-item>
            <n-card
              title="校园网使用指南"
              hoverable
              @click="open_guide"
              class="my-card"
            >
              信息办的微信公众号出的。但是没有任何有用信息。
            </n-card>
          </n-grid-item>
          <n-grid-item>
            <n-card
              title="更改校园网密码"
              hoverable
              @click="open_changepassword"
              class="my-card"
            >
              给你跳转到校园网后台修改密码的地方。
            </n-card>
          </n-grid-item>
          <n-grid-item>
            <n-card
              title="打开配置文件夹"
              hoverable
              @click="open_config"
              class="my-card"
            >
              如果你想看都存了些什么的话，或者想自己改。
            </n-card>
          </n-grid-item>
        </n-grid>
      </n-space>
      <n-float-button
        :right="20"
        :bottom="20"
        type="primary"
        @click="showModal = true"
      >
        <n-icon>
          <ColorPaletteOutline />
        </n-icon>
      </n-float-button>
      <n-modal v-model:show="showModal">
        <n-card style="margin: auto 50px">
          <n-button
            strong
            secondary
            type="primary"
            @click="set_background_image"
          >
            设置背景图片
          </n-button>
          <n-button
            strong
            secondary
            type="primary"
            style="margin-left: 50px"
            @click="reset_background_image"
          >
            去掉背景图片
          </n-button>
          <p>
            背景图片透明度: <br /><br /><n-slider
              :on-dragend="set_background_transparence"
              v-model:value="transparence"
              :default-value="0"
              :step="1"
            />
          </p>
          <p>
            背景图片模糊程度: <br /><br /><n-slider
              :on-dragend="set_background_blur"
              v-model:value="blur"
              :default-value="0"
              :step="1"
            />
          </p>
          <p>~以上设置右键刷新页面生效~</p>
        </n-card>
      </n-modal>
    </div>
  </n-scrollbar>
</template>

<style scoped>
.container {
  overflow: auto;
  padding: 10px;
}
.my-switch {
  margin-top: 10px;
}
.my-button {
  margin-top: 5px;
}
.my-card {
  background-color: rgba(255, 255, 255, 0);
  cursor: pointer;
}

.my-card:active {
  transition: transform 0.1s ease-in-out;
  transform: scale(0.99); /* 点击时缩小按钮 */
}
</style>
