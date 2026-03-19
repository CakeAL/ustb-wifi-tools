<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useMessage } from "naive-ui";
import { useLoadingBar } from "naive-ui";
import { onMounted, ref } from "vue";

const loadingBar = useLoadingBar();
// const user_name = ref("");
const pop_message = useMessage();
// const account_flow = ref<Flow | null>(null);
const ammeter_number = ref("");
const ammeter_data = ref(0);
const raw_url = ref("");
const vpn_url = ref("");

onMounted(() => {
  load_ammeter_number();
});

const load_ammeter_number = async () => {
  let res =
    (await invoke("load_setting").catch((err) =>
      pop_message.error(err)
    )) as string;
  if (res.length > 0) {
    let settings = JSON.parse(res);
    ammeter_number.value = settings.ammeter_number;
  }
};

// const load_user_flow = async () => {
//   if (user_name.value.length == 0) return;
//   loadingBar.start();
//   let res = await invoke("load_user_flow", { account: user_name.value }).catch(
//     (err) => pop_message.error(err),
//   );
//   //   console.log(res as string);
//   account_flow.value = JSON.parse(res as string);
//   loadingBar.finish();
// };

const load_ammeter = async () => {
  let number = parseInt(ammeter_number.value);
  if (!isNaN(number)) {
    loadingBar.start();
    let res = await invoke("load_ammeter", {
      ammeterNumber: number,
    }).catch((err) => pop_message.error(err));
    loadingBar.finish();
    ammeter_data.value = res as number;
  } else {
    pop_message.error("电表号应该是纯数字!");
  }
};

const translate_up = async (rawUrl: string) => {
  if (rawUrl.length == 0) {
    vpn_url.value = "";
  }
  let res = await invoke("translate_up", { rawUrl }).catch((err) =>
    pop_message.error(err)
  );
  vpn_url.value = res as string;
};

const translate_down = async (vpnUrl: string) => {
  if (vpnUrl.length == 0) {
    raw_url.value = "";
  }
  let res = await invoke("translate_down", { vpnUrl }).catch((err) =>
    pop_message.error(err)
  );
  raw_url.value = res as string;
};

const copyToClipboard = async (str: string) => {
  await writeText(str)
    .then(() => {
      pop_message.success("成功复制到剪切板");
    })
    .catch((err) => pop_message.error(err));
};
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 其他小工具 </n-text>
    </n-h2>
    <!-- <n-card title="查一下别人当月流量" hoverable class="my-card">
      <p>如果你不在校园网，应先登录为“我不在校园网”模式。</p>
      <n-input
        v-model:value="user_name"
        type="text"
        placeholder="学号/工号"
        @blur="load_user_flow"
        round
      />
      <template #footer v-if="account_flow">
        这个人 ipv4 用了
        {{ (account_flow.data.v4 / 1024).toFixed(2) }} GB，ipv6 用了
        {{ (account_flow.data.v6 / 1024).toFixed(2) }} GB
      </template>
</n-card> -->
    <n-card title="查一下电费" hoverable class="my-card">
      <n-input
        v-model:value="ammeter_number"
        type="text"
        placeholder="电表号"
        @blur="load_ammeter"
        round
      />
      <template #footer v-if="ammeter_data">
        还剩 {{ ammeter_data }} kW·h
      </template>
    </n-card>
    <n-card title="WebVPN 转换" hoverable class="my-card">
      <n-grid x-gap="12" :cols="2">
        <n-gi>
          <n-input
            v-model:value="raw_url"
            type="textarea"
            :placeholder="raw_url.length === 0 ? '原始网址' : raw_url"
            @input="translate_up(raw_url)"
            @focus="raw_url.length !== 0 ? copyToClipboard(raw_url) : 0"
          />
        </n-gi>
        <n-gi>
          <n-input
            v-model:value="vpn_url"
            type="textarea"
            :placeholder="vpn_url.length === 0 ? 'elib网址' : vpn_url"
            @input="translate_down(vpn_url)"
            @focus="vpn_url.length !== 0 ? copyToClipboard(vpn_url) : 0"
          />
        </n-gi>
      </n-grid>
      <template #footer>
        用来把一个链接转换成校内 elib/n.ustb.edu.cn
        的网址，或者转换回来。以便于在校外轻松访问校内资源，或者在校内访问校外资源（🤔）。
      </template>
    </n-card>
  </div>
</template>

<style scoped></style>
