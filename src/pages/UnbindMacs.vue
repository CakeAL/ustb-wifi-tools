<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, onMounted, ref } from "vue";
import { useMessage } from "naive-ui";
import { open } from "@tauri-apps/plugin-shell";
import { CheckmarkCircleOutline, CloseCircleOutline } from "@vicons/ionicons5";
import { store } from "../store";
interface MacAddress {
  device_name: string;
  mac_address: string;
  custom_name: string;
  unbind: boolean;
}

interface ThisMacAddress {
  iface_name: String;
  mac_address: String;
}

const pop_message = useMessage();
const this_mac = ref<ThisMacAddress[]>([]);
const mac_addrs = ref<MacAddress[]>([]);

onMounted(() => {
  get_current_device_mac();
  load_mac_address();
});

const get_current_device_mac = async () => {
  let res = (await invoke("get_current_device_mac").catch((err) =>
    pop_message.error(err)
  )) as string;
  console.log(res);

  this_mac.value = JSON.parse(res);
};

const load_mac_address = async () => {
  let res = await invoke("load_mac_address").catch((err) =>
    pop_message.error(err)
  );
  mac_addrs.value = JSON.parse(res as string);
};

const unbind = async () => {
  //  传入 false 的
  let macs: string[] = mac_addrs.value
    .filter((mac) => !mac.unbind)
    .map((mac) => mac.mac_address);
  // console.log(macs);
  if (macs.length === mac_addrs.value.length) {
    pop_message.warning("你还没选任何要解绑的 MAC 地址😭");
    return;
  }
  await invoke("do_unbind_macs", {
    macs: macs,
  }).catch((err) => pop_message.error(err));
  setTimeout(load_mac_address, 100);
};

const set_mac_custom_name = async (mac: string, index: number) => {
  if (mac_addrs.value[index].custom_name === "") {
    return;
  }
  await invoke("set_mac_custom_name", {
    mac,
    name: mac_addrs.value[index].custom_name,
  }).catch((err) => pop_message.error(err));
};

const unbind_cur_device = async () => {
  let macs = mac_addrs.value
    .filter(
      (mac) =>
        !this_mac.value.map((mac) => mac.mac_address).includes(mac.mac_address)
    )
    .map((mac) => mac.mac_address);
  if (macs.length === mac_addrs.value.length) {
    pop_message.warning(
      "没有与之相匹配的 MAC 地址，可能由于当前账号在此电脑上没有登录过🤔"
    );
    return;
  }
  // console.log(macs);
  await invoke("do_unbind_macs", {
    macs: macs,
  }).catch((err) => pop_message.error(err));
  setTimeout(load_mac_address, 100);
};

const whether_login_cur_device = computed(() => {
  let macs = mac_addrs.value
    .filter((mac) =>
      this_mac.value.map((mac) => mac.mac_address).includes(mac.mac_address)
    )
    .map((mac) => mac.mac_address);
  return macs.length > 0 ? true : false;
});
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 解绑 MAC 地址 </n-text>
    </n-h2>

    <n-card hoverable class="my-card">
      <n-collapse>
        <n-collapse-item title="当前设备 MAC 地址（仅供参考）" name="1">
          <n-popover trigger="hover" placement="top-start">
            <template #trigger>
              <n-statistic label="">
                <span v-for="(mac, index) in this_mac" :key="index">{{ mac.iface_name }}: {{ mac.mac_address
                  }}<br /></span>
              </n-statistic>
            </template>
            如果把该地址解绑会导致立刻断网！其实就是注销登录罢了。<br />
            最前面的是网络接口，如果你的电脑有多个网卡，那么也会有多行。<br />
            一般来说，Windows 设备上 "WLAN"，macOS 设备上 "en0"
            是无线网卡的接口。
          </n-popover>
          <template #header-extra>
            当前设备是否与 {{ store.userName }} 绑定：
            <n-icon-wrapper :size="24" :border-radius="12" v-if="whether_login_cur_device">
              <n-icon :size="24" :component="CheckmarkCircleOutline" />
            </n-icon-wrapper>
            <n-icon-wrapper :size="24" :border-radius="12" color="#F2C97D" icon-color="#000" v-else>
              <n-icon :size="24" :component="CloseCircleOutline" />
            </n-icon-wrapper>
          </template>
        </n-collapse-item>
      </n-collapse>
    </n-card>
    <n-table :bordered="false" style="background-color: transparent">
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
        <tr v-for="(mac_addr, index) in mac_addrs" :key="index" :class="this_mac
            .map((mac) => mac.mac_address)
            .includes(mac_addr.mac_address)
            ? 'highlight-row'
            : ''
          ">
          <th>{{ index + 1 }}</th>
          <th>{{ mac_addr.device_name }}</th>
          <th>
            <n-input v-model:value="mac_addr.custom_name" type="text"
              @blur="set_mac_custom_name(mac_addr.mac_address, index)" />
          </th>
          <th>{{ mac_addr.mac_address }}</th>
          <th style="text-align: center">
            <n-checkbox size="large" v-model:checked="mac_addr.unbind" />
          </th>
        </tr>
      </tbody>
    </n-table>
    <n-grid :x-gap="12" :y-gap="8" :cols="2" style="margin-top: 10px">
      <n-grid-item>
        <n-popover trigger="hover" placement="top-start">
          <template #trigger>
            <n-button strong secondary type="info" @click="unbind_cur_device" style="width: 100%">
              一键解绑当前设备
            </n-button></template>此选项会自动匹配当前设备MAC地址以及校园网记录的MAC地址，并解绑当前设备的MAC地址；<br />也就是“注销登录”</n-popover></n-grid-item>
      <n-grid-item>
        <n-button strong secondary type="primary" @click="unbind" style="width: 100%">
          确定解绑
        </n-button></n-grid-item>
    </n-grid>
    <n-card title="说明" hoverable class="my-card">
      <n-p>上面标黄的一栏是当前设备可能的 MAC
        地址。可以直接点击蓝色按钮注销/解绑当前设备。</n-p>
      <n-p>MAC Address是什么？简单来说校园网靠这个来识别是否是你的设备。</n-p>
      <n-p>
        所以随机 MAC
        地址开启的话，在你再次登录的时候，你的设备新生成了一个虚拟的 MAC
        地址，就会导致你之前的设备被顶掉，详情可看B站视频：<a @click="open('https://www.bilibili.com/video/av792486473/')" style="
            text-underline-offset: 5px;
            text-decoration: underline;
            cursor: pointer;
          ">BV1JC4y1S7WS</a>
        <!-- 点 bv 会打开 av 的网页🤔 -->
      </n-p>
      <n-p>
        校园网后台（因为年久失修）的设备名十有八九是获取不到的，但是本软件提供了自定义设备名功能。这样我们就可以标记我们认识的设备了，并且配置文件可以通过
        Onedrive 进行同步！如果你知道它本机的MAC地址是什么的话。
      </n-p>
      <n-collapse>
        <n-collapse-item title="macOS / iOS 固定 MAC 地址方法" name="1">
          <div>
            <p>
              将私有 Wi-Fi 地址设置为<b>“关闭”</b>（注意，不是固定，macOS
              15以上）
            </p>
            <n-image width="550" src="/7a35cda1e5187cf409a1677bf0c58fa3.png" />
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
</template>

<style scoped>
.my-card {
  margin: 10px 0;
  width: 100%;
  background: rgba(255, 255, 255, 0.1);
}

.highlight-row> :first-child {
  /* border-left: 1px solid #f2c97d;
  border-top: 1px solid #f2c97d; */
  border-bottom: 1px solid #f2c97d;
  /* border-radius: 0 0 0 12px; */
  box-shadow: inset 0px -80px 0px 0px rgba(242, 201, 125, 0.1);
}

.highlight-row> :not(:first-child):not(:last-child) {
  /* border-top: 1px solid #f2c97d; */
  border-bottom: 1px solid #f2c97d;
  box-shadow: inset 0px -80px 0px 0px rgba(242, 201, 125, 0.1);
}

.highlight-row> :last-child {
  /* border-right: 1px solid #f2c97d;
  border-top: 1px solid #f2c97d; */
  border-bottom: 1px solid #f2c97d;
  /* border-radius: 0 0 12px 0; */
  box-shadow: inset 0px -80px 0px 0px rgba(242, 201, 125, 0.1);
}
</style>
