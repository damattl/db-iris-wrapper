import type { MessageView } from "@/api";
import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";

interface MessageViewTableProps {
  messages: MessageView[];
}

export function MessageViewTable({ messages }: MessageViewTableProps) {
  /*
  export type MessageView = {
      id: string;
      train_id: string;
      train: string;
      valid_from?: string | null;
      valid_to?: string | null;
      priority?: number | null;
      category?: string | null;
      code?: number | null;
      timestamp: string;
      m_type?: string | null;
  };
 */

  return (
    <div className="overscroll-x-auto">
      <DataTable
        emptyMessage="Keine Einträge vorhanden"
        // rowClassName={rowClassName}
        size="small"
        value={messages}
        tableStyle={{ minWidth: "50rem" }}
      >
        <Column field="code" header="Code"></Column>

        <Column field="timestamp" header="Timestamp"></Column>
        <Column field="valid_from" header="Gültig ab"></Column>
        <Column field="valid_to" header="Gültig bis"></Column>
      </DataTable>
    </div>
  );
}
