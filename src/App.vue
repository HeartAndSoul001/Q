<template>
  <el-container class="app-container">
    <el-header>
      <el-image style="width: 4%;" :src="appico" fit="fill"/>
    </el-header>
    <el-container class="mid-area">
      <el-aside>
        <el-scrollbar>
          <el-menu>
            <el-sub-menu index="1">
              <template #title>
                <font-awesome-icon icon="fa-solid fa-toolbox" size="lg" fixed-width pull="left" /> IP工具
              </template>
              <el-menu-item-group>
                <el-menu-item index="1-1"><font-awesome-icon icon="fa-solid fa-calculator" size="lg" fixed-width
                    pull="left" />掩码计算</el-menu-item>
                <el-menu-item index="1-2"><font-awesome-icon icon="fa-solid fa-retweet" size="lg" fixed-width
                    pull="left" />格式转换</el-menu-item>
                <el-menu-item index="1-3"><font-awesome-icon icon="fa-solid fa-object-group" size="lg" fixed-width
                    pull="left" />网段处理</el-menu-item>
                <el-menu-item index="1-4"><font-awesome-icon icon="fa-solid fa-binoculars" size="lg" fixed-width
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
      <el-main>this is main</el-main>
    </el-container>
    <el-footer>this is footer</el-footer>
  </el-container>
</template>

<script lang="ts" setup>
import appico from "./assets/app.ico"
</script>

<style scoped>
.app-container {
  height: 100%;
  background-color: white;
}

.el-header {
  height: 10%;
  border-bottom: 1cap;

}

.mid-area {
  height: 80%;
}

.el-footer {
  height: 10%;
  text-align: center;
  background-color: rgb(11, 196, 229);
}

.el-aside {
  width: 20%;
  text-align: center;
}

.el-main {
  text-align: center;
}
</style>
