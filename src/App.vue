<template>
  <el-container class="app-container">
    <el-header></el-header>
    <el-container class="mid-area">
      <el-aside>
        <el-scrollbar>
          <el-menu :router="true">
            <el-sub-menu index="1">
              <template #title>
                <font-awesome-icon icon="fa-solid fa-toolbox" size="lg" fixed-width pull="left" /> IP工具
              </template>
              <el-menu-item-group>
                <el-menu-item index="1-1" :route="{ path: '/' }">
                  <font-awesome-icon icon="fa-solid fa-calculator" size="lg" fixed-width pull="left" />子网计算器
                </el-menu-item>
                <el-menu-item index="1-2" :route="{ path: '/greet' }">
                  <font-awesome-icon icon="fa-solid fa-retweet" size="lg" fixed-width pull="left" />格式转换
                </el-menu-item>
                <!-- 其他菜单项 -->
              </el-menu-item-group>
            </el-sub-menu>
            <!-- 其他子菜单 -->
          </el-menu>
        </el-scrollbar>
      </el-aside>
      <el-main>
        <router-view></router-view>
      </el-main>
    </el-container>
    <el-footer>
      <div class="text-center py-6 text-xs">
        <p>Made with ❤️ by WZQ</p>
      </div>
    </el-footer>
  </el-container>
</template>

<script lang="ts" setup>
import 'element-plus/es/components/tree-select/src/tree-select-option.mjs';
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