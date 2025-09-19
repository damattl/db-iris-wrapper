import type { MovementView, StopView } from "@/api";
import { getTimestampNotNull } from "@/utils/date";
import {
  getPlatform,
  displayTime,
  sortByArrival,
  displayTimeCurrent,
} from "@/utils/stop";
import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";

interface StopViewTableProps {
  stops: StopView[];
  nextStop?: StopView | null;
}

const platformTemplate = (stop: StopView) => {
  return getPlatform(stop);
};

const movementTemplate = (movement?: MovementView | null) => {
  const showCurrent = movement?.current && movement.current != movement.planned;
  const currentColor =
    getTimestampNotNull(movement?.current) >
    getTimestampNotNull(movement?.planned)
      ? "text-red-500"
      : "text-green-500";

  return (
    <div className="font-mono">
      <span>{displayTime(movement)}</span>
      {showCurrent && (
        <span className={currentColor}> ({displayTimeCurrent(movement)})</span>
      )}
    </div>
  );
};

const arrivalTemplate = (stop: StopView) => {
  return movementTemplate(stop.arrival);
};

const departureTemplate = (stop: StopView) => {
  return movementTemplate(stop.departure);
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
      emptyMessage="Keine Einträge vorhanden"
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
