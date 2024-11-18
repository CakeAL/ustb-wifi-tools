<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useMessage } from "naive-ui";
import { open } from "@tauri-apps/plugin-shell";

interface MacAddress {
  device_name: string;
  mac_address: string;
  custom_name: string;
}

const pop_message = useMessage();
const this_mac = ref<string>("");
const mac_addrs = ref<MacAddress[] | null>(null);
const unbind_macs = ref<Array<boolean>>([]);
const custom_names = ref<Array<string>>([]);

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
      custom_names.value.push(mac_addrs.value[i].custom_name);
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

const set_mac_custom_name = async (mac: string, index: number) => {
  if (custom_names.value[index] === "") {
    return;
  }
  await invoke("set_mac_custom_name", {
    mac,
    name: custom_names.value[index],
  }).catch((err) => pop_message.error(err));
};
</script>

<template>
  <n-scrollbar style="max-height: 100vh">
    <div class="container">
      <n-h2 prefix="bar" type="success" style="margin-top: 15px">
        <n-text type="success"> 解绑 MAC 地址 </n-text>
      </n-h2>
      <p>MAC Address是什么？简单来说校园网靠这个来识别是否是你的设备。</p>
      <p>
        所以随机MAC地址开启的话，就会导致你之前的设备被顶掉，详情可看B站视频：<a
          @click="open('https://www.bilibili.com/video/av792486473/')"
          style="
            text-underline-offset: 5px;
            text-decoration: underline;
            cursor: pointer;
          "
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
              <th>校园网后台设备名</th>
              <th>自定义设备名</th>
              <th>MAC Address</th>
              <th>是否解绑</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(mac_addr, index) in mac_addrs" :key="index">
              <th>{{ index + 1 }}</th>
              <th>{{ mac_addr.device_name }}</th>
              <th>
                <n-input
                  v-model:value="custom_names[index]"
                  type="text"
                  @blur="set_mac_custom_name(mac_addr.mac_address, index)"
                />
              </th>
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
      <p>
        校园网后台的设备名十有八九是获取不到的，但是现在可以自定义设备名啦。这样我们就可以标记我们认识的设备了，如果你知道它本机的MAC地址是什么的话。
      </p>
      <n-card hoverable>
        <n-collapse>
          <n-collapse-item title="macOS / iOS 固定 MAC 地址方法" name="1">
            <div>
              <p>
                将私有 Wi-Fi 地址设置为<b>“关闭”</b>（注意，不是固定，macOS
                15以上）
              </p>
              <n-image
                width="550"
                src="/7a35cda1e5187cf409a1677bf0c58fa3.png"
              />
            </div>
          </n-collapse-item>
          <n-collapse-item title="安卓/鸿蒙设备固定 MAC 地址方法" name="2">
            <div>
              <p>WLAN 详情 -> 隐私 -> 使用设备 MAC 地址</p>
              <n-image width="300" src="/QQ_1731405540124.png" />
            </div>
          </n-collapse-item>
          <n-collapse-item title="Windows 设备固定 MAC 地址方法" name="3">
            <div>
              <p>网络和 Internet -> WLAN -> 属性 -> 随机硬件地址 -> 关</p>
              <n-image width="550" src="/QQ_1731406861238.png" />
            </div>
          </n-collapse-item>
        </n-collapse>
      </n-card>
    </div>
  </n-scrollbar>
</template>

<style scoped>
.container {
  overflow: auto;
  padding: 10px;
}
</style>
