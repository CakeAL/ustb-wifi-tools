<script setup lang="ts">
import { mb2gb, min2hour } from "../helper";
import { UserLoginLog } from "../pages/UserLoginLog.vue";

const props = defineProps<{
  title?: String;
  user_log: UserLoginLog | null;
}>();
</script>

<template>
  <n-thing
    :title="title"
    content-style="margin-top: 10px;"
    style="margin-top: 10px"
  >
    <template #description>
      <n-table
        :bordered="false"
        :single-line="false"
        striped
      >
        <thead>
          <tr>
            <th>ipv4 ⬇</th>
            <th>ipv4 ⬆</th>
            <th>ipv6 ⬇</th>
            <th>ipv6 ⬆</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>{{ mb2gb(user_log?.ipv4_down) }} GB</td>
            <td>{{ mb2gb(user_log?.ipv4_up) }} GB</td>
            <td>{{ mb2gb(user_log?.ipv6_down) }} GB</td>
            <td>{{ mb2gb(user_log?.ipv6_up) }} GB</td>
          </tr>
          <tr>
            <td>💰 花费:</td>
            <td>🕙 使用时长:</td>
            <td>🛜 消耗流量:</td>
            <td></td>
          </tr>
          <tr>
            <td>{{ user_log?.cost.toFixed(2) }} 元</td>
            <td>
              {{
                min2hour(
                  user_log?.used_duration,
                )
              }} h
            </td>
            <td>{{ user_log?.used_flow.toFixed(2) }} MB</td>
            <td></td>
          </tr>
        </tbody>
      </n-table>
    </template>
  </n-thing>
</template>
