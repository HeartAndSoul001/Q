<template>
  <el-container class="app-container">
    <el-header></el-header>
    <el-container class="mid-area">
      <el-aside>
        <el-scrollbar>
          <el-menu>
            <el-sub-menu index="1">
              <template #title>
                <font-awesome-icon icon="fa-solid fa-toolbox" size="lg" fixed-width pull="left" /> IP工具
              </template>
              <el-menu-item-group>
                <el-menu-item index="1-1" @click="content=subnetsCalcu"><font-awesome-icon icon="fa-solid fa-calculator" size="lg" fixed-width
                    pull="left" />掩码计算</el-menu-item>
                <el-menu-item index="1-2" @click="content=formTrans"><font-awesome-icon icon="fa-solid fa-retweet" size="lg" fixed-width
                    pull="left" />格式转换</el-menu-item>
                <el-menu-item index="1-3" @click="content=networkHandle"><font-awesome-icon icon="fa-solid fa-object-group" size="lg" fixed-width
                    pull="left" />网段处理</el-menu-item>
                <el-menu-item index="1-4" @click="content=ipDetect"><font-awesome-icon icon="fa-solid fa-binoculars" size="lg" fixed-width
                    pull="left" />地址探测</el-menu-item>
              </el-menu-item-group>
            </el-sub-menu>
            <el-sub-menu index="2">
              <template #title>
                <font-awesome-icon icon="fa-solid fa-file-contract" size="lg" fixed-width pull="left" />合同管理
              </template>
              <el-menu-item-group>
                <el-menu-item index="2-1">合同录入</el-menu-item>
                <el-menu-item index="2-2">合同执行</el-menu-item>
                <el-menu-item index="2-3">合同归档</el-menu-item>
              </el-menu-item-group>
            </el-sub-menu>
          </el-menu>
        </el-scrollbar>
      </el-aside>
      <el-main><{{ content }} /></el-main>
    </el-container>
    <el-footer>
      <div class="text-center py-6 text-xs">
        <p>Made with ❤️ by WZQ</p>
      </div>
    </el-footer>
  </el-container>
</template>

<script lang="ts" setup>
import subnetsCalcu from './components/subnetsCalcu.vue';
import formTrans from './components/formTrans.vue';
import networkHandle from './components/networkHandle.vue';
import ipDetect from './components/ipDetect.vue';

const content = subnetsCalcu

</script>

<style scoped>
.app-container {
  height: 100%;
  background-color: #ffffff;
}

.el-header {
  height: 10%;
  border-bottom: solid 1px #dcdfe6;
  font-size: 16px;
}

.mid-area {
  height: 80%;
}

.el-footer {
  height: 10%;
  text-align: center;
  border-top: solid 1px #dcdfe6;
  font-size: 14px;
  font-weight: 500;
}

.el-aside {
  width: 15%;
  text-align: center;
  border-right: solid 1px #dcdfe6;
}

.el-menu {
  font-weight: 600;
}

.el-main {
  text-align: center;
  font-size: 16px;
}
</style>
