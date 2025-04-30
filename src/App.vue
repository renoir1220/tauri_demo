// src/App.vue
<script setup lang="ts">
import { ref, computed } from 'vue';
// 1. 导入子组件和类型
import AddQuery from './components/AddQuery.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

import type { QuerySetData, TableRowData } from './types'; // 假设类型定义在 src/types.ts

// --- Reactive State ---

// 控制新增/修改模态框的显示
const isAddQueryVisible = ref(false);

// 查询列表数据 (用实际数据或初始化为空数组)
const queryList = ref<QuerySetData[]>([]);

// 当前选中的查询项数据 (存储数据对象，而非 DOM 元素)
const selectedQueryData = ref<QuerySetData | null>(null);

// 右侧面板标题
const rightPanelTitleText = ref('查询结果 / 导入数据预览');

// 控制右侧是显示表格还是占位符
const isTableVisible = ref(false);

// 明确 tableData 的类型，例如 Record<string, any>[] 或更具体的 TableRowData[]
const tableData = ref<TableRowData[]>([]);
const isLoading = ref(false); // 添加一个加载状态

// --- Computed Properties ---

// 根据当前状态计算占位符应显示的文本
const placeholderContent = computed(() => {
  if (isLoading.value) {
    return '正在加载数据...';
  }
  if (selectedQueryData.value) {
    return `已选择 "${selectedQueryData.value.name}"，请点击“执行查询”`;
  }
  return '请先执行查询或导入数据';
});

// --- Methods ---

// 打开新增/修改查询模态框
function openAddModifyQueryModal() {
  // TODO: 如果是修改，可能需要传递 selectedQueryData 到模态框
  isAddQueryVisible.value = true;
  console.log('模态框打开:openAddModifyQueryModal'); // 会在 WebView DevTools Console 显示
}

// 处理模态框关闭事件
function handleModalClose() {
  isAddQueryVisible.value = false;
}

// 处理模态框保存事件
function handleModalSave(formData: QuerySetData) {
  console.log('App.vue: Received data via @save event:', formData);
  // TODO: 调用 Rust 后端 API 保存查询 (使用 tauri api 的 invoke)
  queryList.value.push(formData);
  isAddQueryVisible.value = false; // 关闭模态框
}

// 处理点击查询列表项
function handleQueryItemClick(query: QuerySetData) {
  selectedQueryData.value = query; // 更新选中的数据状态
  rightPanelTitleText.value = `已选择: ${query.name}`;
  isTableVisible.value = false; // 选择后先隐藏表格，等待执行
}

// 处理点击“执行查询”按钮
function handleExecuteQuery() {
  if (!selectedQueryData.value) {
    alert('请先从左侧列表中选择一个查询！');
    isTableVisible.value = false;
    return;
  }
  rightPanelTitleText.value = `查询结果: ${selectedQueryData.value.name}`;
  console.log('执行查询:', selectedQueryData.value.query);
  // TODO: 调用 Rust 后端执行查询 selectedQueryData.value.query
  // const results = await invoke('execute_query', { sql: selectedQueryData.value.query });
  // tableData.value = results; // 更新表格数据
  // 模拟显示表格和一些假数据
  tableData.value = [
    { id: 'BL2001', diagnosis: '(胃) 腺癌...', ihc: 'CK(AE1/AE3)(+)...', t: 'T4a', n: 'N3', m: 'M1' },
    { id: 'BL2002', diagnosis: '(胃) 低分化腺癌...', ihc: 'HER2(2+)...', t: 'T3', n: 'N1', m: 'Mx' },
  ]; // 实际应填充后端返回数据
  isTableVisible.value = true;
}

// 处理点击“导入数据”按钮
async function handleImportData() {
  selectedQueryData.value = null; // 清除查询选择
  rightPanelTitleText.value = '导入数据';
  isTableVisible.value = false; // 清除旧表格
  isLoading.value = true;

  try {
    // 2. 使用 Tauri dialog API 打开文件选择框
    const selectedPath = await open({
      multiple: false, // 只允许选一个文件
      directory: false, // 不允许选文件夹
      filters: [{ // 文件过滤器
        name: 'Excel 工作簿',
        extensions: ['xlsx', 'xls']
      }]
    });

    if (typeof selectedPath === 'string') {
      // 3. 如果用户选择了文件，调用 Rust 后端命令处理文件
      rightPanelTitleText.value = '正在解析 Excel 文件...';
      console.log('Selected file path:', selectedPath);

      // 调用 Rust 命令 (假设叫 'import_excel_data') 并传递路径
      const importedData = await invoke<TableRowData[]>('import_excel_data', {
         filePath: selectedPath
      });

      // 6. 更新前端状态
      tableData.value = importedData;
      rightPanelTitleText.value = `导入数据预览: ${selectedPath.split(/[\\/]/).pop()}`; // 显示文件名
      isTableVisible.value = true; // 显示表格

    } else {
      // 用户取消了选择
      console.log('用户取消了文件选择');
      rightPanelTitleText.value = '查询结果 / 导入数据预览'; // 恢复标题
    }
  } catch (error) {
    console.error('导入或解析 Excel 文件失败:', error);
    alert(`导入失败: ${error}`);
    rightPanelTitleText.value = '导入错误'; // 显示错误状态
  } finally {
    isLoading.value = false;
  }
}

// 处理点击“生成测试数据”按钮
function handleGenerateData() {
  if (!isTableVisible.value) { // 检查是否有数据（表格是否可见）
    alert('请先执行查询或导入数据，再生成测试数据！');
    return;
  }
  console.log('触发生成测试数据流程...');
  // TODO: 可能需要打开一个新的模态框或 Tauri 窗口来进行字段映射配置
  alert('在此处将弹出“映射管理”对话框 (原型中未实现)');
  // TODO: 调用 Rust 后端处理 tableData.value 来生成数据
}


</script>

<template>
  <div class="flex flex-col h-screen p-4 space-y-4 bg-gray-100">
    <div class="flex-shrink-0 bg-white p-3 rounded-lg shadow space-x-2">
      <button @click="openAddModifyQueryModal"
        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 transition duration-150 ease-in-out">
        新增/修改查询
      </button>
      <button @click="handleExecuteQuery"
        class="px-4 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-300 transition duration-150 ease-in-out">
        执行查询
      </button>
      <button @click="handleImportData"
        class="px-4 py-2 bg-yellow-500 text-white rounded-md hover:bg-yellow-600 focus:outline-none focus:ring-2 focus:ring-yellow-300 transition duration-150 ease-in-out">
        导入数据(Excel)
      </button>
      <button @click="handleGenerateData"
        class="px-4 py-2 bg-purple-500 text-white rounded-md hover:bg-purple-600 focus:outline-none focus:ring-2 focus:ring-purple-300 transition duration-150 ease-in-out">
        生成测试数据
      </button>
    </div>

    <div class="flex flex-grow space-x-4 overflow-hidden">
      <div class="w-1/4 flex-shrink-0 bg-white p-4 rounded-lg shadow flex flex-col">
        <h3 class="text-lg font-semibold mb-3 border-b pb-2 text-gray-700">查询列表</h3>
        <ul class="space-y-1 overflow-y-auto flex-grow">
          <li v-for="query in queryList" :key="query.id"
            class="query-item p-2 rounded-md hover:bg-gray-100 cursor-pointer transition duration-150 ease-in-out"
            :class="{ 'selected': selectedQueryData === query }" @click="handleQueryItemClick(query)">
            {{ query.name }}
          </li>
        </ul>
      </div>

      <div class="flex-grow bg-white p-4 rounded-lg shadow flex flex-col overflow-hidden">
        <h3 class="text-lg font-semibold mb-3 border-b pb-2 text-gray-700 flex-shrink-0">{{ rightPanelTitleText }}</h3>
        <div class="flex-grow overflow-auto">
          <div v-if="!isTableVisible" class="text-gray-500 text-center pt-10">
            {{ placeholderContent }}
          </div>
          <table v-show="isTableVisible && !isLoading" class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  ID</th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  诊断</th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  IHC</th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">T
                </th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">N
                </th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">M
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr v-for="(row, index) in tableData" :key="index">
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ row.id || row.colA }}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ row.diagnosis || row.colB }}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ row.ihc || row.colC }}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ row.t }}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ row.n }}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ row.m }}</td>
              </tr>
              <tr v-if="tableData.length === 0">
                <td colspan="6" class="px-6 py-4 text-center text-sm text-gray-500">没有数据</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <AddQuery v-if="isAddQueryVisible" @close="handleModalClose" @save="handleModalSave" />
  </div>
</template>

<style scoped>
/* Style for selected list item */
/* .query-item 类现在直接在 <li> 上， scoped 会生效 */
.query-item.selected {
  background-color: #e0f2fe;
  /* Light blue background for selected */
  font-weight: 600;
}

/* 如果需要全局字体，应放在 main.css/style.css 中 */
/* body { font-family: 'Inter', sans-serif; } */
</style>