import type { MessageView, StationView, StatusCodeView } from "@/api";
import { displayDateTime, getTimestampNotNull } from "@/utils/date";
import { useRouter } from "@tanstack/react-router";
import { Column } from "primereact/column";
import { DataTable, type DataTableRowClickEvent } from "primereact/datatable";

interface MessageViewTableProps {
  messages: MessageView[];
  codes: StatusCodeView[];
  stations: StationView[];
}

export function MessageViewTable({
  messages,
  codes,
  stations,
}: MessageViewTableProps) {
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

    const url = router.buildLocation({
      to: `/trains/${message.train_id}`,
    }).href;
    window.open(url, "_blank", "noopener,noreferrer");
  };

  return (
    <div className="overscroll-x-auto">
      <DataTable
        onRowClick={handleRowClick}
        emptyMessage="Keine Einträge vorhanden"
        // rowClassName={rowClassName}
        size="small"
        value={messages.sort(
          (msgA, msgB) =>
            getTimestampNotNull(msgA.timestamp) -
            getTimestampNotNull(msgB.timestamp),
        )}
        rowHover
        tableStyle={{ minWidth: "50rem" }}
      >
        <Column className="w-[50px]" field="code" header="Code"></Column>
        <Column
          className="w-[200px]"
          body={(row: MessageView) => {
            return codes.find((c) => c.code === row.code)?.long_text || "";
          }}
          header="Beschreibung"
        ></Column>
        <Column
          className="w-[200px]"
          body={(row: MessageView) => displayDateTime(row.timestamp)}
          header="Erstellt (Europe/Berlin)"
        ></Column>
        <Column
          className="w-[200px]"
          body={(row: MessageView) => displayDateTime(row.last_updated)}
          header="Zuletzt aktualisiert"
        ></Column>
        <Column
          body={(row: MessageView) => {
            const stationNames = row.stations
              .map((id) => stations?.find((s) => s.id === id)?.name || "")
              .join(", ");
            return stationNames || "Keine Haltestellen";
          }}
          header="Haltestellen"
        ></Column>
        <Column className="w-[180px]" field="train_id" header="Zug ID"></Column>
      </DataTable>
    </div>
  );
}
