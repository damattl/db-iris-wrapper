import type { StopView } from "@/api";
import { getPlatform, displayTime, sortByArrival } from "@/utils/stop";
import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";

interface StopViewTableProps {
  stops: StopView[];
  nextStop?: StopView | null;
}

const platformTemplate = (stop: StopView) => {
  return getPlatform(stop);
};

const arrivalTemplate = (stop: StopView) => {
  return <span className="font-mono">{displayTime(stop.arrival)}</span>;
};

const departureTemplate = (stop: StopView) => {
  return <span className="font-mono">{displayTime(stop.departure)}</span>;
};

export function StopViewTable({ stops, nextStop }: StopViewTableProps) {
  sortByArrival(stops);

  const rowClassName = (row: StopView) => {
    return {
      "font-bold": row.id == nextStop?.id,
    };
  };

  return (
    <DataTable
      rowClassName={rowClassName}
      size="small"
      value={stops}
      tableStyle={{ minWidth: "50rem" }}
    >
      <Column
        style={{ width: "40%" }}
        field="station.name"
        header="Station"
      ></Column>
      <Column
        style={{ width: "20%" }}
        body={arrivalTemplate}
        header="Ankunft"
      ></Column>
      <Column
        style={{ width: "20%" }}
        body={departureTemplate}
        header="Abfahrt"
      ></Column>
      <Column
        style={{ width: "20%" }}
        body={platformTemplate}
        header="Platform"
      ></Column>
    </DataTable>
  );
}
