<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useMessage } from "naive-ui";

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

interface Flow {
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
  let res = await invoke("load_user_flow_by_state").catch((err) =>
    pop_message.error(err)
  );
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

const mb2gb = (mb: number | undefined) => {
  if (mb === undefined) return 0;
  else return parseFloat((mb / 1024).toFixed(2));
};

const remain_flow = computed(() => {
  if (
    account_info.value?.note.leftFlow !== undefined &&
    account_flow.value?.data.v4 !== undefined
  ) {
    let remain =
      parseFloat(account_info.value?.note.leftFlow) -
      account_flow.value?.data.v4;
    return parseFloat((remain / 1024).toFixed(2));
  }
});

const remain_percentage = computed(() => {
    if (
    account_info.value?.note.leftFlow !== undefined &&
    account_flow.value?.data.v4 !== undefined
  ) {
    let per =
      (parseFloat(account_info.value?.note.leftFlow) -
      account_flow.value?.data.v4) / parseFloat(account_info.value?.note.leftFlow);
    return parseFloat((per * 100).toFixed(2));
  }
});
</script>

<template>
  <div class="container" v-if="account_info !== null">
    <h2>当前账号使用详情</h2>
    <p>用户类别：{{ account_info.note.service }}</p>
    <p>当前余额：{{ account_info.note.leftmoeny }}</p>
    <p>是否在线：{{ if_online }}</p>
    <p>用户状态：{{ account_info.note.status }}</p>
    <p>更新日期：{{ account_info.serverDate }}</p>
    <br />
    <p>当前流量使用情况：</p>
    <p>
      ipv4 下行：{{ account_flow?.data.v4 }} MB，约合
      {{ mb2gb(account_flow?.data.v4) }} GB
    </p>
    <p>
      ipv6 下行(不计费)：{{ account_flow?.data.v6 }} MB，约合
      {{ mb2gb(account_flow?.data.v6) }} GB
    </p>
    <p>当前剩余 ipv4 下行流量：{{ remain_flow }} GB，大概是 {{ remain_percentage }} %</p>
    <br/>
    <p>Tips: </p>
    <p>ipv6 上下行，ipv4 上行都是不计费的~</p>
    <p>ipv4 下行超出 120 GB 的部分大约 0.6 RMB/GB </p>
  </div>
</template>

<style scoped></style>
