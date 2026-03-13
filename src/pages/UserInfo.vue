<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import dayjs from "dayjs";
import { useMessage } from "naive-ui";
import { computed, onMounted, ref } from "vue";
import { mb2gb } from "../helper";
import { store, UserDashboard } from "../store";

const pop_message = useMessage();

onMounted(() => {
  if (store.userDashboard === undefined) {
    refresh_user_dashboard();
  }
});

const refresh_user_dashboard = async () => {
  let res = await invoke("refresh_user_dashboard").catch((err) =>
    pop_message.error(err)
  );

  store.userDashboard = JSON.parse(res as string) as UserDashboard;
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
            <n-grid x-gap="12" :cols="4">
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
                  {{ store.userDashboard.leftMoney }}
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
                      }} GB
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
                      }} GB
                    </n-statistic>
                  </template>
                  {{ store.userDashboard.chinanetDownFlow }} MB
                </n-popover>
              </n-gi>
            </n-grid>
          </template>
          当前剩余 ipv4 下行流量：{{ mb2gb(store.userDashboard.leftFlow) }}
          GB，大概是
          {{ remain_percentage }} %
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
