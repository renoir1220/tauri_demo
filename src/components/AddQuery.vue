// src/components/QueryFormModal.vue
<script setup lang="ts">
import { ref } from 'vue';
import type { QuerySetData } from '../types'; // <--- 导入类型

// 定义该组件可以触发的事件：'close' 和 'save'
// 'save' 事件会携带表单数据作为参数
// 为 emit 添加类型约束
const emit = defineEmits<{
  (e: 'close'): void; // close 事件没有参数
  (e: 'save', payload: QuerySetData): void; // save 事件携带 QueryFormData 类型的 payload
}>();

// --- 表单相关的状态和逻辑会在这里 ---
const queryName = ref('');
const querySql = ref('');

function handleSave() {
  // 创建符合接口的对象
  const formData: QuerySetData = {
    id: Date.now().toString(),
    name: queryName.value,
    query: querySql.value
  };
  emit('save', formData); // 现在 payload 会被检查是否符合 QueryFormData
}

function handleCancel() {
  // 触发 'close' 事件，通知父组件关闭模态框
  emit('close');
}
</script>

<template>
  <div class="fixed ... flex items-center justify-center ...">
    <div class="relative mx-auto p-5 border w-full max-w-xl shadow-lg rounded-md bg-white">
       <div class="mt-3 text-center">
        <h3 class="text-lg leading-6 font-medium text-gray-900">新增/修改查询</h3>
        <div class="mt-2 px-7 py-3">
          <p class="text-sm text-gray-500">
            在这里放置你的表单输入框 (input, textarea 等)。
          </p>
          <input type="text" placeholder="查询名称" class="mt-2 px-3 py-2 border rounded w-full" 
          v-model="queryName" />
          <textarea placeholder="查询语句 (SQL 或其他)" class="mt-2 px-3 py-2 border rounded w-full h-24"
          v-model="querySql">
        </textarea>
        </div>
        <div class="items-center px-4 py-3 space-x-2">
          <button
            @click="handleSave"
            class="px-4 py-2 bg-green-500 text-white text-base font-medium rounded-md w-auto shadow-sm hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-300"
          >
            保存
          </button>
          <button
            @click="handleCancel"
            class="px-4 py-2 bg-gray-300 text-gray-800 text-base font-medium rounded-md w-auto shadow-sm hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300"
          >
            取消
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 可以添加一些模态框特定的样式 */
</style>