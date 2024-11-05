<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useMessage } from "naive-ui";
import { open } from "@tauri-apps/plugin-shell";

interface MacAddress {
  device_name: string;
  mac_address: string;
}

const pop_message = useMessage();
const this_mac = ref<string>("");
const mac_addrs = ref<MacAddress[] | null>(null);
const unbind_macs = ref<Array<boolean>>([]);

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
  if (mac_addrs.value !== null) {
    for (let i = 0; i < mac_addrs.value.length; i += 1) {
      unbind_macs.value.push(false);
    }
  }
};

const unbind = async () => {
  // console.log(unbind_macs.value); 传入 false 的
  let macs: string[] = [];
  let i: number;
  for (i = 0; i < unbind_macs.value.length; i += 1) {
    if (unbind_macs.value[i] === false && mac_addrs.value !== null) {
      macs.push(mac_addrs.value[i].mac_address);
    }
  }
  // console.log(macs);
  await invoke("do_unbind_macs", {
    macs: macs,
  }).catch((err) => pop_message.error(err));
  setTimeout(load_mac_address, 100);
};
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 解绑 MAC 地址 </n-text>
    </n-h2>
    <p>MAC Address是什么？简单来说校园网靠这个来识别是否是你的设备。</p>
    <p>
      所以随机MAC地址开启的话，就会导致你之前的设备被顶掉，详情可看B站视频：<a
        @click="open('https://www.bilibili.com/video/av792486473/')"
        style="text-underline-offset: 5px; text-decoration: underline; cursor: pointer;"
        >BV1JC4y1S7WS</a
      >
      <!-- 点 bv 会打开 av 的网页🤔 -->
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
            <th>
              <n-checkbox size="large" v-model:checked="unbind_macs[index]" />
            </th>
          </tr>
        </tbody>
      </n-table>
      <br />
      <n-button strong secondary type="info" @click="unbind">
        确定解绑
      </n-button>
    </div>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
  margin: 5px;
}
</style>
