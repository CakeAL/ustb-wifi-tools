<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui";
import { ref, onMounted } from "vue";

const pop_message = useMessage();
const site_num = ref<number>(1);
const options = [
  { label: "北科 内网", value: 1 },
  { label: "中科大 ipv6", value: 2 },
  { label: "东北大学 ipv6", value: 3 },
  { label: "南京大学 ipv6", value: 4 },
  { label: "上海大学 ipv6", value: 5 },
  { label: "江苏大学 ipv6", value: 6 },
];
const ipv4_address = ref<string>("");
const ipv6_address = ref<string>("");

onMounted(() => {
  load_ip_address();
});

const open_speed_test = async () => {
  await invoke("open_speed_test", { siteNum: site_num.value }).catch((err) =>
    pop_message.error(err)
  );
};

const load_ip_address = async () => {
  let res = (await invoke("load_ip_address").catch((err) =>
    pop_message.error(err)
  )) as string;
  let resp: [string, string] = JSON.parse(res);
  if (resp[0] === "") {
    ipv4_address.value = "当前无公网 ipv4 地址，但貌似仍可使用北科内网测速。";
  } else {
    ipv4_address.value = resp[0];
  }
  if (resp[1] === "") {
    ipv6_address.value =
      "当前无公网 ipv6 地址，但貌似仍可使用北科内网测速，无法使用其他 ipv6 测速站点。";
  } else {
    ipv6_address.value = resp[1];
  }
};
</script>

<template>
  <div>
    <n-space vertical>
      <n-h2 prefix="bar" type="success" style="margin-top: 15px">
        <n-text type="success"> 测个速，不费校园网流量的 </n-text>
      </n-h2>
      <n-select v-model:value="site_num" :options="options" />
      <n-button strong secondary type="primary" @click="open_speed_test">
        点我
      </n-button>
      <h3>当前您的公网地址是：</h3>
      <p>ipv4: {{ ipv4_address }}</p>
      <p>ipv6: {{ ipv6_address }}</p>
    </n-space>
  </div>
</template>

<style scoped></style>
