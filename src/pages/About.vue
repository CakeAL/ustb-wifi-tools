<script setup lang="ts">
import { ref } from "vue";

const show = ref(false);
const show1 = ref(false);
</script>

<template>
  <div class="container">
    <n-h2 prefix="bar" type="success" style="margin-top: 15px;">
      <n-text type="success">
        关于作者以及 Tips
      </n-text>
    </n-h2>
    <n-card hoverable class="my-card">
      <n-space vertical>
        <p>更方便的获取 USTB 校园网每日使用情况，查询流量，解绑 MAC 地址等。</p>
        <p>
          通过调用校园网的
          API，并且数据都在本地进行存储。
        </p>
        <p>
          源代码可以在 https://github.com/CakeAL/ustb-wifi-tools/
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
          <n-image
            width="200"
            src="/4243a50e827cbd72c4213247dd0833bb_720.jpg"
          />
        </n-collapse-transition>
        <h4>关于作者：一个来自USTB计院物联网工程的老东西。</h4>
        <p>这是我的Github：https://github.com/CakeAL/ </p>
        <p>这是我的bilibili: https://space.bilibili.com/13161874</p>
        <h4>下面是学校给你的TIPS</h4>
        <p>校园网老是要重新登陆：</p>
        <n-switch v-model:value="show1">
          <template #checked> 折叠 </template>
          <template #unchecked> 展开 </template>
        </n-switch>
        <n-collapse-transition :show="show1">
          <n-image
            width="500"
            src="/A250E7A1CDCE4161894E703B177AC0C6_频繁掉线.jpg"
          />
        </n-collapse-transition>
        <p>USTB-V6接入说明：http://cipp.ustb.edu.cn/app.IAManage/index.jsp?m=iamanage&c=Notice&a=noticeInfo&id=22</p>
        <p>一个原来能看校园网出口出入流量的网站，现在已经打不开了：http://traffic.ustb.edu.cn/</p>
        <h4>校园网 Radius 超时可能解决方法：</h4>
        <p>0. 尝试手动打开http://login.ustb.edu.cn/登录</p>
        <p>1. 不要使用注销页直接登陆，首先忽略此网络（忘记），然后重开Wi-Fi，再进行登陆，会跳出登录页（虽然ip和注销页是一样的）</p>
        <p>2. 关掉代理，再重新进行上述操作。</p>
        <p>3. 重启电脑。</p>
        <p>注销页和登录页网址：http://202.204.48.66/ 或者 http://login.ustb.edu.cn/ (你知道么，学号是可以点击的)</p>
        <h4>给校园网省流方法（偏向使用 ipv6 的方法）：</h4>
        <p>1. 设置 ipv6 DNS服务器，保证 ipv6-test 能全部通过。一般浏览器都是默认优先 ipv6 的</p>
        <p>2. Steam：使用 Dogfight360 提供的工具，对 Steam 进行 ipv6 代理。</p>
        <p>3. 软件镜像源，使用带 ipv6 的镜像源，比如清华源（一般会自动选择 ipv6） mirros6 </p>
        <p>4. 通过 pt 站下载大型软件（比如 Photoshop 等）</p>
        <p>5. 自建/使用 ipv6 代理服务器</p>
        <br/>
        <p>当前 version = "0.7.4"</p>
    </n-space>
    </n-card>
  </div>
</template>

<style scoped>
.container {
  height: 100vh;
  overflow: auto;
  margin: 5px;
}

.my-card {
  background: rgba(255, 255, 255, 0.05);
}
</style>
