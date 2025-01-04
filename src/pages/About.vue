<script setup lang="ts">
import { open } from "@tauri-apps/plugin-shell";
import { ref } from "vue";

const show = ref(false);
const show1 = ref(false);
</script>

<template>
  <div>
    <n-h2 prefix="bar" type="success" style="margin-top: 15px">
      <n-text type="success"> 关于作者以及 Tips </n-text>
    </n-h2>
    <n-card title="关于此软件" hoverable class="my-card">
      <p>更方便的获取 USTB 校园网每日使用情况，查询流量，解绑 MAC 地址等。</p>
      <p>通过调用校园网的 API，并且数据都在本地进行存储。</p>
      <p>
        源代码可以在
        <a @click="open('https://github.com/CakeAL/ustb-wifi-tools/')" style="
            text-decoration: underline;
            text-underline-offset: 5px;
            cursor: pointer;
          ">github.com/CakeAL/ustb-wifi-tools/</a>
        查看，欢迎Star，Folk并提交PR
      </p>
      <p>对此软件有任何问题可以在 Github 项目提交 issues 。</p>
      <p>已经懒得改前端了。</p>
      <p>如果觉得这个软件帮助了您许多，可以打钱给我：</p>
      <n-switch v-model:value="show">
        <template #checked> 折叠收款码 </template>
        <template #unchecked> 展开收款码 </template>
      </n-switch>
      <n-collapse-transition :show="show">
        <n-image width="200" src="/4243a50e827cbd72c4213247dd0833bb_720.jpg" />
      </n-collapse-transition>
    </n-card>
    <n-card title="关于作者" hoverable class="my-card">
      <p>关于作者：一个来自USTB计院物联网工程的老东西。</p>
      <p>这是我的Github：https://github.com/CakeAL/</p>
      <p>这是我的bilibili: https://space.bilibili.com/13161874</p>
    </n-card>
    <n-card title="学校给你的 TIPS" hoverable class="my-card">
      <p>校园网老是要重新登陆：</p>
      <n-switch v-model:value="show1">
        <template #checked> 折叠 </template>
        <template #unchecked> 展开 </template>
      </n-switch>
      <n-collapse-transition :show="show1">
        <n-image width="500" src="/A250E7A1CDCE4161894E703B177AC0C6_频繁掉线.jpg" />
        <p>参考本软件“解绑 MAC 地址” 页。</p>
      </n-collapse-transition>
      <p style="
          text-decoration: underline;
          text-underline-offset: 5px;
          cursor: pointer;
        " @click="
          open(
            'http://cipp.ustb.edu.cn/app.IAManage/index.jsp?m=iamanage&c=Notice&a=noticeInfo&id=22'
          )
          ">
        USTB-V6接入说明
      </p>
      <p>
        一个原来能看校园网出口出入流量的网站，现在已经打不开了：http://traffic.ustb.edu.cn/
      </p>
    </n-card>
    <n-card title="校园网 Radius 超时可能解决方法" hoverable class="my-card">
      <p>
        0.
        点击本应用登录页的登录校园网功能，或尝试手动打开http://login.ustb.edu.cn/登录
      </p>
      <p>
        1.
        不要使用注销页直接登陆，首先忽略此网络（忘记），然后重开Wi-Fi，再进行登陆，会跳出登录页（虽然ip和注销页是一样的）
      </p>
      <p>2. 关掉代理，再重新进行上述操作。</p>
      <p>3. 重启电脑。</p>
      <p>
        注销页和登录页网址：<a @click="open('http://202.204.48.66/')" style="
            text-decoration: underline;
            text-underline-offset: 5px;
            cursor: pointer;
          ">http://202.204.48.66/</a>
        或者
        <a @click="open('http://login.ustb.edu.cn/')" style="
            text-decoration: underline;
            text-underline-offset: 5px;
            cursor: pointer;
          ">http://login.ustb.edu.cn/</a>
        (你知道么，学号是可以点击的)
      </p>
    </n-card>
    <n-card title="给校园网省流方法（偏向使用 ipv6 的方法）" hoverable class="my-card">
      <p>
        1. 设置 ipv6 DNS服务器，保证 ipv6-test
        能全部通过。一般浏览器都是默认优先 ipv6 的
      </p>
      <p>2. Steam：使用 Dogfight360 提供的工具，对 Steam 进行 ipv6 代理。</p>
      <p>
        3. 软件镜像源，使用带 ipv6 的镜像源，比如清华源（一般会自动选择 ipv6）
        mirros6
      </p>
      <p>4. 通过 pt 站下载大型软件（比如 Photoshop 等）</p>
      <p>5. 自建/使用 ipv6 代理服务器</p>
    </n-card>
    <n-card title="软件版本信息" hoverable class="my-card">
      <n-p>
        当前 version = "1.1.3" |
        <a @click="open('https://github.com/CakeAL/ustb-wifi-tools/blob/main/CHANGELOG.md')" style="
            text-decoration: underline;
            text-underline-offset: 5px;
            cursor: pointer;
          ">历史更新记录</a> </n-p><n-p> Copyright © 2024 CakeAL All Rights Reserved </n-p>
    </n-card>
  </div>
</template>

<style scoped>
.my-card {
  margin-top: 10px;
  width: 100%;
  background: rgba(255, 255, 255, 0.1);
}
</style>
