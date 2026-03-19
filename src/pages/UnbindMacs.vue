<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { CheckmarkCircleOutline, CloseCircleOutline } from "@vicons/ionicons5";
import { CloseOutline } from "@vicons/ionicons5";
import { useMessage } from "naive-ui";
import { computed, onMounted, ref } from "vue";
import { store } from "../store";

// [
//     "0", // 是否在线
//     "MACADDRESS",
//     "#PC",
//     "2026-03-15 11:23:32", // 最后一次登录
//     "10.24.18.81", // 最后一次登录的 IP
//     "否", // 是否是哑终端
//     "" // 自定义别名
// ],
interface MacList {
  total: number;
  rows: Array<Array<string>>;
}

interface ThisMacAddress {
  iface_name: string;
  mac_address: string;
}

const pop_message = useMessage();
const this_mac = ref<ThisMacAddress[]>([]);
const mac_list = ref<MacList | undefined>(undefined);
const ajaxCsrfToken = ref<string>("");

onMounted(() => {
  get_current_device_mac();
  load_mac_address();
});

const get_current_device_mac = async () => {
  let res =
    (await invoke("get_current_device_mac").catch((err) =>
      pop_message.error(err)
    )) as string;
  console.log(res);

  this_mac.value = JSON.parse(res);
};

const load_mac_address = async () => {
  let res = await invoke("load_mac_address").catch((err) => {
    if (err === "EOF while parsing a value at line 1 column 0") {
      err = "没有找到任何数据";
    }
    pop_message.error(err);
  });
  let json_value = JSON.parse(res as string);
  mac_list.value = json_value[0];
  ajaxCsrfToken.value = json_value[1];
};

const unbind = async (mac: string) => {
  await invoke("do_unbind_mac", {
    mac,
    ajaxCsrfToken: ajaxCsrfToken.value,
  }).catch((err) => pop_message.error(err));
  setTimeout(load_mac_address, 100);
};

const set_mac_custom_name = async (
  macAddress: string,
  terminalName: string,
) => {
  await invoke("set_mac_custom_name", {
    macAddress,
    terminalName,
    ajaxCsrfToken: ajaxCsrfToken.value,
  }).catch((err) => pop_message.error(err));
  setTimeout(load_mac_address, 100);
};

const unbind_cur_device = async () => {
  let macs = mac_list.value?.rows
    .filter(
      (mac) => !this_mac.value.map((mac) => mac.mac_address).includes(mac[1]),
    )
    .map((mac) => mac[1]);
  if (macs && macs.length === mac_list.value?.rows.length) {
    pop_message.warning(
      "没有与之相匹配的 MAC 地址，可能由于当前账号在此电脑上没有登录过🤔",
    );
    return;
  } else if (macs) {
    unbind(macs[0]);
  }
};

const whether_login_cur_device = computed(() => {
  let macs = mac_list.value?.rows
    .filter((mac) =>
      this_mac.value.map((mac) => mac.mac_address).includes(mac[1])
    )
    .map((mac) => mac[1]);
  return macs != undefined && macs.length > 0 ? true : false;
});
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 解绑 MAC 地址 </n-text>
    </n-h2>

    <n-card hoverable class="my-card" style="margin-bottom: 10px">
      <n-collapse>
        <n-collapse-item title="当前设备 MAC 地址（仅供参考）" name="1">
          <n-popover trigger="hover" placement="top-start">
            <template #trigger>
              <n-statistic label="">
                <span v-for="(mac, index) in this_mac" :key="index">{{
                    mac.iface_name
                  }}: {{ mac.mac_address }}<br /></span>
              </n-statistic>
            </template>
            如果把该地址解绑会导致立刻断网！其实就是注销登录罢了。<br />
            最前面的是网络接口，如果你的电脑有多个网卡，那么也会有多行。<br />
            一般来说，Windows 设备上 "WLAN"，macOS 设备上 "en0"
            是无线网卡的接口。
          </n-popover>
          <template #header-extra>
            当前设备是否与 {{ store.userName }} 绑定：
            <n-icon-wrapper
              :size="24"
              :border-radius="12"
              v-if="whether_login_cur_device"
            >
              <n-icon :size="24" :component="CheckmarkCircleOutline" />
            </n-icon-wrapper>
            <n-icon-wrapper
              :size="24"
              :border-radius="12"
              color="#F2C97D"
              icon-color="#000"
              v-else
            >
              <n-icon :size="24" :component="CloseCircleOutline" />
            </n-icon-wrapper>
          </template>
        </n-collapse-item>
      </n-collapse>
    </n-card>
    <n-table
      :bordered="false"
      style="background-color: transparent; margin-bottom: 10px"
    >
      <thead>
        <tr>
          <th>序号</th>
          <th>设备类型</th>
          <th>自定义设备名</th>
          <th>MAC Address</th>
          <th>解绑</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(mac_addr, index) in mac_list?.rows"
          :key="index"
          :class="
            this_mac
              .map((mac) => mac.mac_address)
              .includes(mac_addr[1])
            ? 'highlight-row'
            : ''
          "
        >
          <th>{{ index + 1 }}</th>
          <th>{{ mac_addr[2] }}</th>
          <th>
            <n-input
              v-model:value="mac_addr[6]"
              type="text"
              @blur="set_mac_custom_name(mac_addr[1], mac_addr[6])"
            />
          </th>
          <th>{{ mac_addr[1] }}</th>
          <th style="text-align: left">
            <n-button
              strong
              secondary
              circle
              type="warning"
              @click="unbind(mac_addr[1])"
            >
              <template #icon>
                <n-icon>
                  <CloseOutline />
                </n-icon>
              </template>
            </n-button>
          </th>
        </tr>
      </tbody>
    </n-table>
    <n-popover trigger="hover" placement="top-start">
      <template #trigger>
        <n-button
          strong
          secondary
          type="info"
          @click="unbind_cur_device"
          style="width: 100%"
        >
          一键解绑当前设备
        </n-button></template>此选项会自动匹配当前设备MAC地址以及校园网记录的MAC地址，并解绑当前设备的MAC地址；<br
      />也就是“注销登录”</n-popover>
    <n-card title="说明" hoverable class="my-card">
      <n-p>上面标黄的一栏是当前设备可能的 MAC
        地址。可以直接点击蓝色按钮注销/解绑当前设备。</n-p>
      <n-p>MAC Address是什么？简单来说校园网靠这个来识别是否是你的设备。</n-p>
      <n-p>
        所以随机 MAC
        地址开启的话，在你再次登录的时候，你的设备新生成了一个虚拟的 MAC
        地址，就会导致你之前的设备被顶掉，详情可看B站视频：<a
          @click="open('https://www.bilibili.com/video/av792486473/')"
          style="text-underline-offset: 5px; text-decoration: underline; cursor: pointer"
        >BV1JC4y1S7WS</a>
        <!-- 点 bv 会打开 av 的网页🤔 -->
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
.highlight-row > :first-child {
  /* border-left: 1px solid #f2c97d;
  border-top: 1px solid #f2c97d; */
  border-bottom: 1px solid #f2c97d;
  /* border-radius: 0 0 0 12px; */
  box-shadow: inset 0px -80px 0px 0px rgba(242, 201, 125, 0.1);
}

.highlight-row > :not(:first-child):not(:last-child) {
  /* border-top: 1px solid #f2c97d; */
  border-bottom: 1px solid #f2c97d;
  box-shadow: inset 0px -80px 0px 0px rgba(242, 201, 125, 0.1);
}

.highlight-row > :last-child {
  /* border-right: 1px solid #f2c97d;
  border-top: 1px solid #f2c97d; */
  border-bottom: 1px solid #f2c97d;
  /* border-radius: 0 0 12px 0; */
  box-shadow: inset 0px -80px 0px 0px rgba(242, 201, 125, 0.1);
}
</style>
