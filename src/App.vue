<template>
  <el-container class="app-container">
    <el-header>this is header</el-header>
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

</script>

<style scoped>
.app-container {
  height: 100%;
}

.el-header {
  /* background: red; */
  height: 10%;
  text-align: center;
  text-align-last: auto;
  border-bottom: 1px;
  background-color: rgb(221, 188, 188);
}

.mid-area {
  height: 80%;
}

.el-footer {
  height: 10%;
  text-align: center;
  border-top: 1px solid #e8eaec;
  background-color: #c53698;
}

.el-aside {
  width: 150px;
  text-align: center;
  border-right: 1px solid #e8eaec;
  background-color: aquamarine;
  margin: 0px;
  padding-left: 0px;
}

.el-main {
  text-align: center;
  border-left: 1px solid #e8eaec;
  border-top: 1px solid #e8eaec;
  border-bottom: 1px solid #e8eaec;
}</style>
