// src/App.vue
<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue';
// 1. 导入子组件和类型
import AddQuery from './components/AddQuery.vue';
import { open, type OpenDialogOptions } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { AgGridVue } from "ag-grid-vue3"; // <--- 导入 AgGridVue 组件
import type { ColDef, GridReadyEvent } from 'ag-grid-community'; // <--- (可选) 导入列定义类型以获得更好的类型提示
import { ClientSideRowModelModule ,themeMaterial,ColumnAutoSizeModule} from 'ag-grid-community';

import type { QuerySetData, TableRowData } from './types'; // 假设类型定义在 src/types.ts


// --- Reactive State ---

const agGridModules = [ClientSideRowModelModule,ColumnAutoSizeModule ];


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

// ---> 新增: AG Grid 的列定义状态 <---
// 列定义需要是响应式的，因为列名是动态的
const columnDefs = ref<ColDef[]>([]);

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

// --- Watchers ---

// ---> 新增: 监视 tableData 的变化，动态生成列定义 <---
watch(tableData, (newTableData) => {
  if (newTableData && newTableData.length > 0) {
    // 从第一行数据获取所有列名 (keys)
    const headers = Object.keys(newTableData[0]);
    console.log("table headers:", headers); // 调试输出列名
    // 将列名映射为 AG Grid 的列定义对象 (ColDef)
    columnDefs.value = headers.map(header => ({
      field: header, // `field` 必须匹配 TableRowData 中的键 (列名)
      headerName: header, // 表头显示名称，默认用字段名
      sortable: true,     // 启用排序
      filter: true,       // 启用筛选 (基础文本筛选)
      resizable: true,    // 允许调整列宽
      // 你可以添加更多配置，比如 width, valueGetter, cellRenderer 等
    }));
    console.log("Generated Column Definitions:", columnDefs.value);
  } else {
    // 如果没有数据，清空列定义
    columnDefs.value = [];
  }
}, { deep: true }); // 使用 deep watch 可能不是最高效的，但对于数组内对象的变化是有效的，也可以只 watch 数组本身



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
  selectedQueryData.value = null;
  rightPanelTitleText.value = '导入数据';
  isTableVisible.value = false;
  isLoading.value = true;
  tableData.value = []; // 清空旧数据
  columnDefs.value = []; // 清空旧列定义

  try {
    const selectedPath = await open({
      multiple: false,
      filters: [{
        name: 'Excel',
        extensions: ['xlsx', 'xls']
      }]
    } as OpenDialogOptions);
    if (typeof selectedPath === 'string' || (Array.isArray(selectedPath) && selectedPath != "")) {
      const filePath = Array.isArray(selectedPath) ? selectedPath[0] : selectedPath;
      rightPanelTitleText.value = '正在解析 Excel 文件...';
      // 调用 Rust
      const importedData = await invoke<TableRowData[]>('import_excel_data', { filePath });

      tableData.value = importedData; // 更新行数据 (触发 watch)

      await nextTick();

      rightPanelTitleText.value = `导入数据预览: ${filePath.split(/[\\/]/).pop()}`;
      isTableVisible.value = true; // 准备显示表格
    } else { /* ... user cancelled ... */ }
  } catch (error) { /* ... error handling ... */ }
  finally { isLoading.value = false; }
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
          <div v-show="isTableVisible && !isLoading" style="height: 100%; width: 100%;">
            <AgGridVue style="height: 100%; width: 100%;" :rowData="tableData" :columnDefs="columnDefs"
              :theme="themeMaterial" :pagination="true" :paginationPageSize="20"
              :paginationPageSizeSelector="[10, 20, 50, 100]" :modules="agGridModules"
              @grid-ready="(params: GridReadyEvent) => params.api.sizeColumnsToFit()">
            </AgGridVue>
          </div>
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