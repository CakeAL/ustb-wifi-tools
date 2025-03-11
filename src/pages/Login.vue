<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { dataDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-shell";
import {
  ArrowForwardCircleOutline,
  ChevronDownOutline,
  ColorPaletteOutline,
} from "@vicons/ionicons5";
import { useLoadingBar, useMessage } from "naive-ui";
import { onMounted, ref } from "vue";
import { railStyle } from "../helper";
import { store } from "../store";
import { check_update } from "../update";

const loadingBar = useLoadingBar();
const pop_message = useMessage();
const sessionid = ref<string>("");
const user_name = ref<string>("");
const password = ref<string>("");
const account = ref<[string, string]>(["", ""]);
const button_disabled = ref<boolean>(false);
const login_state = ref<boolean>(false);
const login_via_vpn = ref<boolean>(false);
const showModal = ref<boolean>(false);
const transparence = ref<number>(0);
const blur = ref<number>(0);
const options = ref<any>([]);

onMounted(() => {
  check_login_state();
  load_setting();
});

const load_setting = async () => {
  let res =
    (await invoke("load_setting").catch((err) =>
      pop_message.error(err)
    )) as string;
  if (res.length > 0) {
    let settings = JSON.parse(res);
    account.value = settings.account;
    transparence.value = settings.background_transparence;
    blur.value = settings.background_blur;

    user_name.value = account.value[0][0];
    password.value = account.value[0][1];

    options.value = account.value.map((account, num) => {
      return {
        label: account[0],
        key: num,
      };
    });
  }
};

const handleSelect = (key: number) => {
  user_name.value = account.value[key][0];
  password.value = account.value[key][1];
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
        store.setUserName(user_name.value);
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
    store.clearUserName();
    pop_message.success(res as string);
  }
};

const set_setting = async () => {
  await invoke("set_setting").catch((err) => pop_message.error(err));
};

const manually_check_update = async () => {
  loadingBar.start();
  await check_update(true);
  loadingBar.finish();
};

const submit_login_ustb_wifi = async () => {
  loadingBar.start();
  await invoke("submit_login_ustb_wifi", {
    userName: user_name.value,
    password: password.value,
  })
    .then((res) => {
      pop_message.success(res as string);
      loadingBar.finish();
    })
    .catch((err) => {
      pop_message.error(err);
      loadingBar.error();
    });
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

const open_microsoft_login = async () => {
  await invoke("open_microsoft_login").catch((e) => pop_message.error(e));
};

const switchModal = ref(false);
const switchToUserName = ref("");
const switchToPassword = ref("");
const handleSelectSwitch = (key: number) => {
  if (account.value[key][0] === store.userName) {
    pop_message.warning("不要选择当前的账号");
    return;
  } else if (store.userName === "") {
    pop_message.warning("当前账号可能为空，请先登录当前账号");
    return;
  }
  switchToUserName.value = account.value[key][0];
  switchToPassword.value = account.value[key][1];
};
const switchLoginUstbWifi = async () => {
  loadingBar.start();

  try {
    await invoke("switch_login_ustb_wifi", {
      userName: switchToUserName.value,
      password: switchToPassword.value,
    });
  } catch (error) {
    pop_message.error(error as string);
    loadingBar.error();
    return;
  }
  loadingBar.finish();
  pop_message.success("切换成功");
};
</script>

<template>
  <div>
    <n-space vertical>
      <div v-if="!login_state">
        <n-h3 prefix="bar" type="success" style="margin-top: 15px">
          输入校园网学号和密码（均在本地存储，也可通过 Onedrive 同步）
        </n-h3>
        <n-grid :x-gap="5" :y-gap="8" :cols="6" style="margin-top: 10px">
          <n-grid-item :span="3">
            <n-input
              v-model:value="user_name"
              type="text"
              placeholder="学号/工号"
              round
            /></n-grid-item>
          <n-grid-item :span="3">
            <n-input
              v-model:value="password"
              type="password"
              show-password-on="mousedown"
              placeholder="密码"
              round
            /></n-grid-item>
          <n-grid-item :span="3">
            <n-button
              strong
              secondary
              type="primary"
              @click="get_cookies"
              :disabled="button_disabled"
            >
              登陆校园网后台获取统计数据 ⭐️
            </n-button>
          </n-grid-item><n-grid-item>
            <n-dropdown
              trigger="hover"
              :options="options"
              @select="handleSelect"
            >
              <n-button type="info">选择账号
                <n-icon size="20">
                  <ChevronDownOutline />
                </n-icon></n-button>
            </n-dropdown></n-grid-item><n-grid-item :span="2">
            <n-switch
              v-model:value="login_via_vpn"
              :rail-style="railStyle"
              class="my-switch"
            >
              <template #checked> 我不在校园网 </template>
              <template #unchecked> 我在校园网 </template>
            </n-switch></n-grid-item>
        </n-grid>
        <n-h3
          prefix="bar"
          type="success"
          style="margin-top: 15px"
          v-if="button_disabled === true"
        >登录中...</n-h3>
      </div>
      <div v-else>
        <n-h3 prefix="bar" type="success" style="margin-top: 15px"
        >您已登录 {{ store.userName }}<br
          />如果其他页面不能获取到信息，请点击登出再重新登录。</n-h3>
      </div>
      <n-grid :x-gap="12" :y-gap="8" :cols="2" style="margin-top: 10px">
        <n-grid-item>
          <n-card title="登出" hoverable @click="logout" class="my-card">
            想换个账号登录校园网后台？
          </n-card>
        </n-grid-item><n-grid-item>
          <n-card
            title="手动检查更新"
            hoverable
            @click="manually_check_update"
            class="my-card"
          >
            手动地检查更新。
          </n-card>
        </n-grid-item><n-grid-item>
          <n-card
            title="登陆校园网"
            hoverable
            @click="submit_login_ustb_wifi"
            class="my-card"
          >
            解决了出现“Radius认证超时！”的问题。<br /> &nbsp;
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card
            title="一键切换登录校园网账号"
            hoverable
            @click="switchModal = true"
            class="my-card"
          >
            会自动注销当前账号校园网登录，并尝试登录你选择的账号。
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card
            title="备份配置文件到 Onedrive"
            hoverable
            @click="open_microsoft_login"
            class="my-card"
          >
            登录微软账号后，会将配置文件上传到 Onedrive 的<b>应用</b>文件夹。
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card
            title="打开配置文件夹"
            hoverable
            @click="open_config"
            class="my-card"
          >
            如果你想看都存了些什么的话，或者想自己改。<br /> &nbsp;
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card
            title="更改校园网密码"
            hoverable
            @click="open_changepassword"
            class="my-card"
          >
            给你跳转到校园网后台修改密码的地方。<br /> &nbsp;
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
        <n-button strong secondary type="primary" @click="set_background_image">
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
        <n-p>
          背景图片透明度: <br /><br /><n-slider
            :on-dragend="set_background_transparence"
            v-model:value="transparence"
            :default-value="0"
            :step="1"
          />
        </n-p>
        <n-p>
          背景图片模糊程度: <br /><br /><n-slider
            :on-dragend="set_background_blur"
            v-model:value="blur"
            :default-value="0"
            :step="1"
          />
        </n-p>
        <n-p style="text-align: center">~ 以上设置右键刷新页面生效 ~</n-p>
      </n-card>
    </n-modal>
    <n-modal v-model:show="switchModal">
      <n-card style="margin: auto 50px">
        <n-p
        >请选择一个账号（如果这里没有，你需要先登录校园网后台获取该账号的统计数据，然后这个选项就有了）：</n-p>
        <n-dropdown
          trigger="hover"
          :options="options"
          @select="handleSelectSwitch"
        >
          <n-button type="info">选择账号
            <n-icon size="20">
              <ChevronDownOutline />
            </n-icon></n-button>
        </n-dropdown>
        <div
          v-if="switchToUserName !== '' && store.userName !== ''"
          style="margin-top: 10px"
        >
          <n-statistic
            label="你确定要切换账号么？此操作会从当前设备注销左侧账号，并尝试登录右侧的账号。"
          >
            {{ store.userName }}
            <n-icon size="24" style="position: relative; top: 3px">
              <ArrowForwardCircleOutline />
            </n-icon>
            {{ switchToUserName }}
          </n-statistic>
          <n-button
            style="margin-top: 10px"
            type="warning"
            @click="switchLoginUstbWifi"
          >是</n-button>
        </div>
      </n-card>
    </n-modal>
  </div>
</template>

<style scoped>
.my-switch {
  margin-top: calc((34px - 22px) / 2);
  float: inline-end;
}

.my-card {
  background-color: rgba(255, 255, 255, 0);
  cursor: pointer;
}

.my-card:hover {
  box-shadow:
    rgba(127, 231, 196, 0.4) 3px 3px,
    rgba(127, 231, 196, 0.3) 6px 6px,
    rgba(127, 231, 196, 0.2) 9px 9px,
    rgba(127, 231, 196, 0.1) 12px 12px,
    rgba(127, 231, 196, 0.05) 15px 15px;
}

.my-card:active {
  transition: transform 0.1s ease-in-out;
  transform: scale(0.99);
  /* 点击时缩小按钮 */
}
</style>
