import type { StopView, TrainView } from "@/api";
import { getTimestamp } from "@/utils/date";
import { arrivalComparer, displayTime } from "@/utils/stop";
import { useRouter } from "@tanstack/react-router";
import { Column } from "primereact/column";
import { DataTable, type DataTableRowClickEvent } from "primereact/datatable";
import { useEffect, useMemo } from "react";

interface TrainViewTableProps {
  stops: StopView[];
  trains: TrainView[];
}

export function TrainViewTable({ trains, stops }: TrainViewTableProps) {
  const router = useRouter();

  useEffect(() => {
    const el = document.querySelector(
      ".p-datatable .p-datatable-tbody .target-row",
    );
    if (el) {
      el.scrollIntoView({
        behavior: "instant",
        block: "start",
      });
    }
  }, []);

  const stopsByTrain = useMemo(() => {
    const map: { [key: string]: StopView | null | undefined } = {};
    for (const stop of stops ?? []) {
      map[stop.train_id] = stop;
    }
    return map;
  }, [stops]);

  const trainsSorted = useMemo(() => {
    const cpy = [...(trains ?? [])];
    cpy.sort((a, b) => {
      const stopA = stopsByTrain[a.id];
      const stopB = stopsByTrain[b.id];

      if (!stopA || !stopB) {
        console.warn("Cant compare, missing stop data");
        return 0;
      }
      return arrivalComparer(stopA, stopB);
    });
    return cpy;
  }, [trains, stopsByTrain]);

  const nextTrain = useMemo(() => {
    const now = Date.now();
    let prev: TrainView | null = null;
    for (const train of trainsSorted) {
      const stop = stopsByTrain[train.id];
      if (!stop) {
        continue;
      }
      const arrival = getTimestamp(stop.arrival?.planned);
      const departure = getTimestamp(stop.departure?.planned);
      if (arrival && arrival < now) {
        prev = train;
      } else if (departure && departure < now) {
        prev = train;
      }
    }
    return prev;
  }, [trainsSorted, stopsByTrain]);

  console.log(stopsByTrain);

  const handleRowClick = (e: DataTableRowClickEvent) => {
    const train = e.data as TrainView;
    router.navigate({ to: `/trains/${train.id}` });
  };

  return (
    <DataTable
      emptyMessage="Keine Einträge vorhanden"
      onRowClick={handleRowClick}
      size="small"
      value={trainsSorted}
      tableStyle={{ minWidth: "50rem" }}
      rowHover
      scrollable
      scrollHeight="500px"
      rowClassName={(rowData) =>
        rowData.id == nextTrain?.id ? "target-row" : ""
      }
    >
      <Column field="number" header="Nummer"></Column>
      <Column field="operator" header="Operator Code"></Column>
      <Column field="category" header="Kategorie"></Column>
      <Column field="line" header="Linie"></Column>
      <Column
        body={(train: TrainView) => {
          const stop = stopsByTrain[train.id];
          return displayTime(stop?.arrival);
        }}
        header="Ankunft"
      ></Column>
      <Column
        body={(train: TrainView) => {
          const stop = stopsByTrain[train.id];
          return displayTime(stop?.departure);
        }}
        header="Abfahrt"
      ></Column>
    </DataTable>
  );
}
