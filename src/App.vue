<template>
  <el-container class="main" style="height: 500px">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu>
          <el-sub-menu index="1">
            <template #title>
              <font-awesome-icon icon="fa-solid fa-toolbox" size="lg" fixed-width /> IP地址处理
            </template>
            <el-menu-item-group>
              <el-menu-item index="1-1"><font-awesome-icon icon="fa-solid fa-calculator" size="lg" fixed-width />掩码计算</el-menu-item>
              <el-menu-item index="1-2"><font-awesome-icon icon="fa-solid fa-retweet" size="lg" fixed-width />格式转换</el-menu-item>
              <el-menu-item index="1-3"><font-awesome-icon icon="fa-solid fa-object-group" size="lg" fixed-width />网段处理</el-menu-item>
              <el-menu-item index="1-4"><font-awesome-icon icon="fa-solid fa-binoculars" size="lg" fixed-width />地址探测</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
          <el-sub-menu index="2">
            <template #title>
              <el-icon><Collection /></el-icon>合同管理
            </template>
            <el-menu-item-group>
              <template #title>Group 1</template>
              <el-menu-item index="2-1">Option 1</el-menu-item>
              <el-menu-item index="2-2">Option 2</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-container>
      <el-header style="text-align: right; font-size: 12px">
        <div class="toolbar">
          <el-dropdown>
            <el-icon style="margin-right: 8px; margin-top: 1px"
              ><setting
            /></el-icon>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item>View</el-dropdown-item>
                <el-dropdown-item>Add</el-dropdown-item>
                <el-dropdown-item>Delete</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          <span>Tom</span>
        </div>
      </el-header>

      <el-main>
        <el-scrollbar>
          <el-table :data="tableData">
            <el-table-column prop="date" label="Date" width="140" />
            <el-table-column prop="name" label="Name" width="120" />
            <el-table-column prop="address" label="Address" />
          </el-table>
        </el-scrollbar>
      </el-main>
    </el-container>
  </el-container>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Menu as IconMenu, SuitcaseLine, Collection, Setting } from '@element-plus/icons-vue'

const item = {
  date: '2016-05-02',
  name: 'Tom',
  address: 'No. 189, Grove St, Los Angeles',
}
const tableData = ref(Array.from({ length: 20 }).fill(item))
</script>

<style scoped>
.main .el-header {
  position: relative;
  background-color: var(--el-color-primary-light-7);
  color: var(--el-text-color-primary);
}
.main .el-aside {
  color: var(--el-text-color-primary);
  background: var(--el-color-primary-light-8);
}
.main .el-menu {
  border-right: none;
}
.main .el-main {
  padding: 0;
}
.main .toolbar {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  right: 20px;
}
</style>
