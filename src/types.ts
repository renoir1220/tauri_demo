// src/types.ts
export interface QuerySetData {
    id: string;
    name: string;
    query: string; // 或者根据你的实际需要调整类型和是否可选 '?'
    // 可以添加 id 等其他字段
  }

export type TableRowData = Record<string, string> ;