<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";
import { useMessage } from "naive-ui";

interface MacAddress {
  device_name: string;
  mac_address: string;
}

const pop_message = useMessage();
const this_mac = ref<string>("");
const mac_addrs = ref<MacAddress[] | null>(null);

onMounted(() => {
  get_current_device_mac();
  load_mac_address();
});

const get_current_device_mac = async () => {
  this_mac.value = (await invoke("get_current_device_mac").catch((err) =>
    pop_message.error(err)
  )) as string;
};

const load_mac_address = async () => {
  let res = await invoke("load_mac_address").catch((err) =>
    pop_message.error(err)
  );
  mac_addrs.value = JSON.parse(res as string);
};
</script>

<template>
  <div class="container">
    <h2>解绑MAC地址</h2>
    <p>MAC Address是什么？简单来说校园网靠这个来识别是否是你的设备。</p>
    <p>
      所以随机MAC地址开启的话，就会导致你之前的设备被顶掉，详情可看B站视频：BV1JC4y1S7WS
    </p>
    <p>当前机器的无线MAC地址是（仅供参考）：{{ this_mac }}</p>
    <p>如果把该地址解绑会导致立刻断网。</p>
    <div v-if="mac_addrs !== null" class="show-data">
      <n-table :bordered="false" :single-line="false">
        <thead>
          <tr>
            <th>序号</th>
            <th>设备名（校园网后台可能获取不到）</th>
            <th>MAC Address</th>
            <th>是否解绑</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(mac_addr, index) in mac_addrs" :key="index">
            <th>{{ index + 1 }}</th>
            <th>{{ mac_addr.device_name }}</th>
            <th>{{ mac_addr.mac_address }}</th>
            <th>1</th>
          </tr>
        </tbody>
      </n-table>
    </div>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
}
</style>
