import type { MessageView, StatusCodeView } from "@/api";
import { useRouter } from "@tanstack/react-router";
import { Column } from "primereact/column";
import { DataTable, type DataTableRowClickEvent } from "primereact/datatable";

interface MessageViewTableProps {
  messages: MessageView[];
  codes: StatusCodeView[];
}

export function MessageViewTable({ messages, codes }: MessageViewTableProps) {
  const router = useRouter();
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

  const handleRowClick = (e: DataTableRowClickEvent) => {
    const message = e.data as MessageView;
    if (!message.train_id) return;

    router.navigate({ to: `/trains/${message.train_id}` });
  };

  return (
    <div className="overscroll-x-auto">
      <DataTable
        onRowClick={handleRowClick}
        emptyMessage="Keine Einträge vorhanden"
        // rowClassName={rowClassName}
        size="small"
        value={messages}
        rowHover
        tableStyle={{ minWidth: "50rem" }}
      >
        <Column field="code" header="Code"></Column>
        <Column
          body={(row: MessageView) => {
            return codes.find((c) => c.code === row.code)?.long_text || "";
          }}
          header="Beschreibung"
        ></Column>
        <Column field="timestamp" header="Timestamp"></Column>
        <Column field="valid_from" header="Gültig ab"></Column>
        <Column field="valid_to" header="Gültig bis"></Column>
        <Column field="train_id" header="Zug ID"></Column>
      </DataTable>
    </div>
  );
}
