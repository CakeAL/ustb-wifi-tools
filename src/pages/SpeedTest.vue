<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
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
const show = ref<boolean>(true);

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
  show.value = false;
};

const copyToClipboard = async (str: string) => {
  await writeText(str)
    .then(() => {
      pop_message.success("成功复制到剪切板");
    })
    .catch((err) => pop_message.error(err));
};

interface IpResultData {
  ip: string;
  dec: string;
  country: string;
  countryCode: string;
  province: string;
  city: string;
  districts: string;
  idc: string;
  isp: string;
  net: string;
  zipcode: string;
  areacode: string;
  protocol: string;
  location: string;
  myip: string;
  time: string;
}

interface IpResult {
  code: number;
  msg: string;
  data: IpResultData;
}

const ip_str = ref("");
const ip_info = ref<IpResult | null>(null);
const get_ip_location = async () => {
  let res = await invoke<string>("get_ip_location", { ip: ip_str.value }).catch(
    (err) => pop_message.error(err)
  );
  ip_info.value = JSON.parse(res as string);
  if (ip_info.value?.code === -1) {
    pop_message.error("未提交查询IP参数");
  } else if (ip_info.value?.code === 201) {
    pop_message.error("IP地址不正确或域名解析失败");
  } else if (ip_info.value?.code === 202) {
    pop_message.error("访问接口超过QPS限制15次/秒, 稍等再查询");
  }
};
</script>

<template>
  <div>
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 测个速，不费校园网流量的 </n-text>
    </n-h2>
    <n-select v-model:value="site_num" :options="options" />
    <n-button
      strong
      secondary
      type="primary"
      @click="open_speed_test"
      style="width: 100%; margin-top: 10px"
    >
      点我
    </n-button>
    <n-spin :show="show">
      <n-card
        title="当前您的公网地址是（点击可复制）"
        hoverable
        class="my-card"
      >
        <n-h6 @click="copyToClipboard(ipv4_address)" style="cursor: pointer"
          >IPv4: {{ ipv4_address }}</n-h6
        >
        <n-h6 @click="copyToClipboard(ipv6_address)" style="cursor: pointer"
          >IPv6: {{ ipv6_address }}</n-h6
        >
      </n-card>
    </n-spin>
    <n-card title="查询 IP 归属地" hoverable class="my-card">
      <n-p
        >服务来自：<a href="https://api.mir6.com">https://api.mir6.com</a></n-p
      >
      <n-input
        v-model:value="ip_str"
        type="text"
        placeholder="IPv4 或 IPv6 地址"
        @blur="get_ip_location"
        round
      />
      <n-grid x-gap="12" y-gap="4" :cols="4" v-if="ip_info" style="margin-top: 10px">
        <n-gi class="my-gi"><n-text type="success"> IP地址</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.ip }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">国家</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.country }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">国家代码</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.countryCode }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">省份</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.province }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">城市</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.city }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">区县</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.districts || "无" }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">IDC服务提供商</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.idc }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">运营商</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.isp }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">网络类型</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.net }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">邮政编码</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.zipcode }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">区号</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.areacode }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">协议</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.protocol }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">归属地</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.location }}</n-gi>
        <n-gi class="my-gi"><n-text type="success">此条数据更新时间</n-text></n-gi>
        <n-gi class="my-gi">{{ ip_info.data.time }}</n-gi>
      </n-grid>
    </n-card>
  </div>
</template>

<style scoped>
.my-card {
  margin: 10px 0;
  width: 100%;
  background: rgba(255, 255, 255, 0.1);
}
.my-gi {
  padding: 2px;
  border: solid 1px rgba(255, 255, 255, 0.3);
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}
</style>
