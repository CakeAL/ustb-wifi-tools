<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui";
import { mb2gb } from "../helper";
import { store } from "../store";

interface Note {
  leftFlow: string;
  leftTime: null | string;
  leftmoeny: string;
  onlinestate: string;
  overdate: string;
  service: string;
  status: string;
  welcome: string;
}

interface Info {
  date: string;
  note: Note;
  outmessage: null | string;
  serverDate: string;
}

interface Data {
  v4: number;
  v6: number;
}

export interface Flow {
  result: number;
  data: Data;
}

const pop_message = useMessage();

const account_info = ref<Info | null>(null);
const account_flow = ref<Flow | null>(null);

onMounted(() => {
  load_refresh_account();
});

const load_refresh_account = async () => {
  let res = await invoke("load_refresh_account").catch((err) =>
    pop_message.error(err)
  );
  account_info.value = JSON.parse(res as string);
  //   console.log(account_info.value);
  if (account_info !== null) {
    setTimeout(() => {
      load_user_flow();
    }, 200);
  }
};

const load_user_flow = async () => {
  console.log(store.userName);

  let res = await invoke("load_user_flow_by_state", {
    userName: store.userName,
  }).catch((err) => pop_message.error(err));
  //   console.log(res as string);
  account_flow.value = JSON.parse(res as string);
};

const if_online = computed(() => {
  if (account_info.value?.note.onlinestate === "1") {
    return "在线";
  } else {
    return "离线";
  }
});

const remain_flow = computed(() => {
  if (
    account_info.value?.note.leftFlow !== undefined &&
    account_flow.value?.data.v4 !== undefined
  ) {
    let remain = Math.max(
      parseFloat(account_info.value?.note.leftFlow) -
      account_flow.value?.data.v4,
      0
    );
    return parseFloat((remain / 1024).toFixed(2));
  }
});

const remain_percentage = computed(() => {
  if (
    account_info.value?.note.leftFlow !== undefined &&
    account_flow.value?.data.v4 !== undefined
  ) {
    let per = Math.max(
      (parseFloat(account_info.value?.note.leftFlow) -
        account_flow.value?.data.v4) /
      parseFloat(account_info.value?.note.leftFlow),
      0
    );
    return parseFloat((per * 100).toFixed(2));
  }
  return 0;
});

const progress_color = computed(() => {
  if (remain_percentage.value > 75) {
    return "#079e5b";
  } else if (remain_percentage.value > 50) {
    return "#0086ed";
  } else if (remain_percentage.value > 25) {
    return "#f49e31";
  } else {
    return "#d43251";
  }
});
</script>

<template>
  <div v-if="account_info !== null">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 当前账号使用详情 </n-text>
    </n-h2>
    <n-list hoverable class="my-list">
      <n-list-item>
        <n-thing :title="store.userName + ' 基本信息'" content-style="margin-top: 10px;">
          <template #description>
            <n-grid x-gap="12" :cols="4">
              <n-gi>
                <p>用户类别:</p>
                <n-tag :bordered="false" type="info">
                  {{ account_info.note.service }}
                </n-tag>
              </n-gi>
              <n-gi>
                <p>当前余额:</p>
                <n-tag :bordered="false" type="info">
                  {{ account_info.note.leftmoeny }}
                </n-tag>
              </n-gi>
              <n-gi>
                <p>是否在线:</p>
                <n-tag :bordered="false" type="info">
                  {{ if_online }}
                </n-tag>
              </n-gi>
              <n-gi>
                <p>用户状态:</p>
                <n-tag :bordered="false" type="info">
                  {{ account_info.note.status }}
                </n-tag>
              </n-gi>
            </n-grid>
          </template>
          更新日期: {{ account_info.serverDate }}
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-progress :color="progress_color" :percentage="remain_percentage" :indicator-text-color="progress_color"
          type="dashboard" gap-position="bottom" class="my-progress" />
        <n-thing title="当月流量使用情况" content-style="margin-top: 10px;">
          <template #description>
            <n-grid x-gap="12" :cols="2">
              <n-gi>
                <n-popover trigger="hover" placement="top-start">
                  <template #trigger>
                    <n-statistic label="ipv4 ⬇">
                      {{ mb2gb(account_flow?.data.v4) }} GB
                    </n-statistic>
                  </template>
                  {{ account_flow?.data.v4 }} MB
                </n-popover>
              </n-gi>
              <n-gi>
                <n-popover trigger="hover" placement="top-start">
                  <template #trigger>
                    <n-statistic label="ipv6 ⬇">
                      {{ mb2gb(account_flow?.data.v6) }} GB
                    </n-statistic>
                  </template>
                  {{ account_flow?.data.v6 }} MB
                </n-popover>
              </n-gi>
            </n-grid>
          </template>
          当前剩余 ipv4 下行流量：{{ remain_flow }} GB，大概是
          {{ remain_percentage }} %
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="Tips" content-style="margin-top: 10px;">
          <template #description>
            <p>ipv6 上下行，ipv4 上行都是不计费的~</p>
            <p>ipv4 下行超出 120 GB 的部分大约 0.6 RMB/GB</p>
          </template>
        </n-thing>
      </n-list-item>
    </n-list>
  </div>
</template>

<style scoped>
.my-progress {
  float: right;
  margin: 10px 5px;
}

.my-list {
  background-color: rgba(240, 248, 255, 0.3);
}

@media (prefers-color-scheme: dark) {
  .my-list {
    background-color: #26262a3d;
  }
}
</style>
