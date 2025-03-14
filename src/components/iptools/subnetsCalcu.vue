<template>
    <div class="subnet-calculator">
      <div class="calculator-header">
        <div class="ip-input-container">
          <div class="ip-segments">
            <div v-for="index in ipParts.length" :key="index-1" class="ip-segment">
              <el-input
                v-model="ipParts[index]"
                :ref="(el: HTMLElement | null) => { if(el) inputRefs[index] = el }"
                @focus="handleFocus(index)"
                @input="validateIpPart(index)"
                @keyup.enter="focusNext(index)"
                @keydown="(e: KeyboardEvent) => handleKeyDown(e, index)"
                @paste.prevent="handlePaste"
                maxlength="3"
                :class="{ 'is-error': ipErrors[index] }"
              />
              <span v-if="index < 3" class="dot">.</span>
            </div>
          </div>
        </div>
      </div>
  
      <div class="table-container">
        <el-table
          v-if="isValidIp"
          :data="subnetTableData"
          border
          :header-cell-style="{
            background: '#f5f7fa',
            color: '#606266',
            textAlign: 'center',
            fontWeight: 'bold'
          }"
          :cell-style="{
            textAlign: 'center'
          }"
          style="width: 100%;"
        >
          <el-table-column prop="prefix" label="子网前缀" min-width="90" />
          <el-table-column prop="netmask" label="子网掩码" min-width="130" />
          <el-table-column prop="wildcard" label="反掩码" min-width="130" />
          <el-table-column prop="network_address" label="子网地址" min-width="130" />
          <el-table-column label="可用地址范围" min-width="260">
            <template #default="scope">
              <span class="address-range">
                {{ scope.row.first_host }} - {{ scope.row.last_host }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="broadcast" label="广播地址" min-width="130" />
          <el-table-column prop="hosts" label="可用地址数" min-width="100" />
        </el-table>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { invoke } from '@tauri-apps/api/tauri'
  
  interface SubnetInfo {
    netmask: string
    wildcard: string
    network_address: string
    first_host: string
    last_host: string
    broadcast: string
    hosts: number
    prefix: number
  }
  
  const ipParts = ref<string[]>(['192', '168', '0', '1'])
  const ipErrors = ref<boolean[]>([false, false, false, false])
  const inputRefs = ref<any[]>([])
  const subnetTableData = ref<SubnetInfo[]>([])
  
  const isValidIp = computed(() => {
    return ipParts.value.every((part, index) => !ipErrors.value[index] && part !== '')
  })
  
  const validateIpPart = (index: number) => {
    const value = ipParts.value[index]
    
    if (value.includes('.')) {
      ipParts.value[index] = value.replace('.', '')
      focusNext(index)
      return
    }
    
    if (!/^\d*$/.test(value)) {
      ipParts.value[index] = value.replace(/\D/g, '')
      return
    }
    
    if (value === '') {
      ipErrors.value[index] = false
      return
    }
    
    const num = parseInt(value)
    ipErrors.value[index] = isNaN(num) || num < 0 || num > 255
    
    if (!ipErrors.value[index] && value.length === 3 && index < 3) {
      focusNext(index)
    }
    
    if (isValidIp.value) {
      calculateSubnets()
    }
  }
  
  const handleKeyDown = (event: KeyboardEvent, index: number) => {
    if (event.key === '.') {
      event.preventDefault()
      focusNext(index)
    }
  }
  
  const handlePaste = (event: ClipboardEvent) => {
    event.preventDefault()
    const text = event.clipboardData?.getData('text') || ''
    const parts = text.split('.')
    
    if (parts.length === 4) {
      parts.forEach((part, index) => {
        const num = parseInt(part)
        if (!isNaN(num) && num >= 0 && num <= 255) {
          ipParts.value[index] = num.toString()
          ipErrors.value[index] = false
        }
      })
      calculateSubnets()
    }
  }
  
  const focusNext = (currentIndex: number) => {
    if (currentIndex < 3) {
      const nextInput = inputRefs.value[currentIndex + 1]
      if (nextInput?.$el instanceof HTMLElement) {
        const input = nextInput.$el.querySelector('input')
        if (input instanceof HTMLInputElement) {
          input.focus()
        }
      }
    }
  }
  
  const handleFocus = (index: number) => {
    const input = inputRefs.value[index]?.$el.querySelector('input')
    if (input instanceof HTMLInputElement) {
      input.setSelectionRange(0, input.value.length)
      input.setAttribute('inputmode', 'numeric')
    }
  }
  
  const calculateSubnets = async () => {
    if (!isValidIp.value) return
    
    const ip = ipParts.value.join('.')
    try {
      const data: SubnetInfo[] = []
      for (let prefix = 32; prefix >= 1; prefix--) {
        const result = await invoke<Omit<SubnetInfo, 'prefix'>>('calculate_subnet_info', { 
          ip, 
          mask: prefix 
        })
        data.push({
          prefix,
          ...result
        })
      }
      subnetTableData.value = data
    } catch (error) {
      console.error('计算子网信息失败:', error)
    }
  }
  
  onMounted(() => {
    if (isValidIp.value) {
      calculateSubnets()
    }
  })
  </script>
  
  <style scoped>
  .subnet-calculator {
    display: flex;
    flex-direction: column;
    padding: 16px;
    background-color: #ffffff;
    box-sizing: border-box;
    min-width: 1200px;
    height: 100%;
    overflow: hidden;
  }
  
  .calculator-header {
    margin-bottom: 16px;
  }
  
  .ip-input-container {
    width: 100%;
    display: flex;
    justify-content: center;
    padding: 12px;
    background: #f5f7fa;
    border-radius: 6px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }
  
  .ip-segments {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
  }
  
  .ip-segment {
    flex: 1;
    max-width: 120px;
    display: flex;
    align-items: center;
  }
  
  .ip-segment :deep(.el-input) {
    width: 100%;
  }
  
  .ip-segment :deep(.el-input__inner) {
    text-align: center;
    font-size: 16px;
    height: 36px;
    padding: 0 4px;
  }
  
  .dot {
    margin: 0 4px;
    font-weight: bold;
    font-size: 18px;
    color: #606266;
  }
  
  .table-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden; /* 添加此行 */
    border-radius: 6px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  }
  
  :deep(.el-table) {
    width: 100%;
  }
  
  :deep(.el-table__cell) {
    padding: 8px;
    white-space: normal;
    word-break: break-word;
  }
  
  :deep(.el-table__body) {
    width: 100% !important;
  }
  
  :deep(.el-table__body-wrapper) {
    overflow-x: hidden;
  }
  
  .address-range {
    font-family: monospace;
    white-space: normal; /* 添加此行 */
    word-break: break-word; /* 添加此行 */
  }
  </style>