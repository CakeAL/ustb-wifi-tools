<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { CloseOutline } from "@vicons/ionicons5";
import dayjs from "dayjs";
import { useMessage } from "naive-ui";
import { computed, onMounted, ref } from "vue";
import { mb2gb, timestamp_format } from "../helper";
import { store, UserDashboard } from "../store";

interface OnlineUser {
  brasid: string;
  downFlow: string;
  hostName: string;
  ip: string;
  loginTime: string;
  mac: string;
  sessionId: string;
  terminalType: string;
  upFlow: string;
  useTime: string;
  userId: number;
}

type LoginHistoryItem = [
  number, // 开始时间戳（毫秒）
  number, // 结束时间戳（毫秒）
  string, // IP地址
  string, // MAC信息
  number, // 使用时长 min
  number, // 使用流量
  number, // 计费方式
  number, // 计费金额
  string | null, // 主机名
  string, // 终端类型
  string, // 设备类型
  number, // 记录ID
];

const pop_message = useMessage();
const online_list = ref<OnlineUser[] | null>(null);
const login_history = ref<LoginHistoryItem[] | null>(null);

onMounted(() => {
  if (store.userDashboard === undefined) {
    refresh_user_dashboard();
  }
  load_online_list();
  load_login_history();
});

const refresh_user_dashboard = async () => {
  let res = await invoke("refresh_user_dashboard").catch((err) =>
    pop_message.error(err)
  );

  store.userDashboard = JSON.parse(res as string) as UserDashboard;
};

const load_online_list = async () => {
  let res =
    (await invoke("load_online_list").catch((err) =>
      pop_message.error(err)
    )) as string;
  online_list.value = JSON.parse(res);
};

const load_login_history = async () => {
  let res =
    (await invoke("load_login_history").catch((err) =>
      pop_message.error(err)
    )) as string;
  login_history.value = JSON.parse(res);
};

const to_offline = async (sessionId: string) => {
  await invoke("do_to_offline", { sessionId }).catch((err) =>
    pop_message.error(err)
  );
  if (online_list.value) {
    online_list.value = online_list.value.filter(item =>
      item.sessionId !== sessionId
    );
  }
};

const remain_percentage = computed(() => {
  if (store.userDashboard !== undefined) {
    return parseFloat(
      ((store.userDashboard.leftFlow / store.userDashboard.userGroup.flowStart)
        * 100).toFixed(2),
    );
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
  <div v-if="store.userDashboard !== undefined">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 当前账号使用详情 </n-text>
    </n-h2>
    <n-list hoverable class="my-list">
      <n-list-item>
        <n-thing
          :title="store.userName + ' 基本信息'"
          content-style="margin-top: 10px;"
        >
          <template #description>
            <n-grid x-gap="12" :cols="2">
              <n-gi>
                <p>用户类别:</p>
                <n-tag :bordered="false" type="info">
                  {{
                    store.userDashboard.serviceDefault
                    .defaultName
                  }}
                </n-tag>
              </n-gi>
              <n-gi>
                <p>当前余额:</p>
                <n-tag :bordered="false" type="info">
                  {{ store.userDashboard.leftMoney }} 元
                </n-tag>
                <!-- </n-gi>
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
                </n-tag> -->
              </n-gi>
            </n-grid>
          </template>
          到期日期: {{
            dayjs(store.userDashboard.invalidDate).format(
              "YYYY-MM-DD",
            )
          }}
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-progress
          :color="progress_color"
          :percentage="remain_percentage"
          :indicator-text-color="progress_color"
          type="dashboard"
          gap-position="bottom"
          class="my-progress"
        />
        <n-thing title="当月流量使用情况" content-style="margin-top: 10px;">
          <template #description>
            <n-grid x-gap="12" :cols="2">
              <n-gi>
                <n-popover trigger="hover" placement="top-start">
                  <template #trigger>
                    <n-statistic label="ipv4 ⬇">
                      {{
                        mb2gb(
                          store.userDashboard
                            .internetDownFlow,
                        )
                      }}
                    </n-statistic>
                  </template>
                  {{ store.userDashboard.internetDownFlow }} MB
                </n-popover>
              </n-gi>
              <n-gi>
                <n-popover trigger="hover" placement="top-start">
                  <template #trigger>
                    <n-statistic label="ipv6 ⬇">
                      {{
                        mb2gb(
                          store.userDashboard
                            .chinanetDownFlow,
                        )
                      }}
                    </n-statistic>
                  </template>
                  {{ store.userDashboard.chinanetDownFlow }} MB
                </n-popover>
              </n-gi>
            </n-grid>
          </template>
          当前剩余 ipv4 下行流量：{{
            mb2gb(store.userDashboard.leftFlow)
          }}，大概是
          {{ remain_percentage }} %
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="当前在线" content-style="margin-top: 10px;">
          <template #description>
            <n-table
              :bordered="false"
              :single-line="false"
              v-if="online_list && online_list.length > 0"
            >
              <thead>
                <tr>
                  <th>上线时间</th>
                  <th>IP地址</th>
                  <th>MAC信息</th>
                  <th>使用时长(mins)</th>
                  <th>使用流量(MB)</th>
                  <th>主机名</th>
                  <th>终端类型</th>
                  <th>注销</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in online_list">
                  <td>{{ user.loginTime }}</td>
                  <td>{{ user.ip }}</td>
                  <td>{{ user.mac }}</td>
                  <td>{{ user.useTime }}</td>
                  <td>{{ user.downFlow }}</td>
                  <td>{{ user.hostName }}</td>
                  <td>{{ user.terminalType }}</td>
                  <td>
                    <n-button
                      strong
                      secondary
                      circle
                      type="warning"
                      @click="to_offline(user.sessionId)"
                    >
                      <template #icon>
                        <n-icon>
                          <CloseOutline />
                        </n-icon>
                      </template>
                    </n-button>
                  </td>
                </tr>
              </tbody>
            </n-table>
            <p v-else>当前没有在线设备</p>
          </template>
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="近期记录" content-style="margin-top: 10px;">
          <template #description>
            <n-table
              :bordered="false"
              :single-line="false"
              v-if="login_history && login_history.length > 0"
            >
              <thead>
                <tr>
                  <th>上线时间</th>
                  <th>注销时间</th>
                  <th>IP地址</th>
                  <th>MAC信息</th>
                  <th>使用时长</th>
                  <th>使用流量</th>
                  <th>计费金额</th>
                  <th>主机名</th>
                  <th>终端类型</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in login_history">
                  <td>{{ timestamp_format(item[0]) }}</td>
                  <td>{{ timestamp_format(item[1]) }}</td>
                  <td>{{ item[2] }}</td>
                  <td>{{ item[3] }}</td>
                  <td>{{ item[4] }}</td>
                  <td>{{ item[5] }}</td>
                  <td>{{ item[7] }}</td>
                  <td>{{ item[8] }}</td>
                  <td>{{ item[9] }}</td>
                </tr>
              </tbody>
            </n-table>
            <p v-else>无近期记录</p>
          </template>
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="Tips" content-style="margin-top: 10px;">
          <template #description>
            <p>ipv6 上下行，ipv4 上行都是不计费的~</p>
            <p>
              ipv4 下行超出 {{ mb2gb(store.userDashboard.userGroup.flowStart) }}
              GB 的部分大约 {{ store.userDashboard.userGroup.flowRate }}
              RMB/MB，即 {{ store.userDashboard.userGroup.flowRate * 1024 }}
              RMB/GB
            </p>
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
