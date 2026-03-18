<script setup lang="ts">
import { mb2gb, min2hour } from "../helper";
import { UserOnlineLogSummary } from "../pages/UserOnlineLog.vue";

const props = defineProps<{
  title?: String;
  summary: UserOnlineLogSummary | undefined;
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
            <td>{{ mb2gb(summary?.INTERNETDOWNFLOW) }}</td>
            <td>{{ mb2gb(summary?.INTERNETUPFLOW) }}</td>
            <td>{{ mb2gb(summary?.CHINANETDOWNFLOW) }}</td>
            <td>{{ mb2gb(summary?.CHINANETUPFLOW) }}</td>
          </tr>
          <tr>
            <td>💰 花费:</td>
            <td>🕙 使用时长:</td>
            <td>🛜 消耗流量:</td>
            <td></td>
          </tr>
          <tr>
            <td>{{ summary?.COSTMONEY }} 元</td>
            <td>
              {{
                min2hour(
                  summary?.TIME,
                )
              }} h
            </td>
            <td>{{ summary?.FLOW }} MB</td>
            <td></td>
          </tr>
        </tbody>
      </n-table>
    </template>
  </n-thing>
</template>
