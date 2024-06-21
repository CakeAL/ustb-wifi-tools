<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { useMessage } from "naive-ui";

const pop_message = useMessage();

const open_speed_test = async () => {
  await invoke("open_speed_test").catch((err) => pop_message.error(err));
};
</script>

<template>
  <div class="container">
    <h2>测个速，不费校园网流量的</h2>
    <n-button strong secondary type="primary" @click="open_speed_test">
      点我
    </n-button>
  </div>
</template>

<style scoped></style>
