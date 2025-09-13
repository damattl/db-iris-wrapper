import { createFileRoute, useRouter } from "@tanstack/react-router";
import {
  stationOptions,
  stopsForStationOptions,
  trainsForStationOptions,
} from "@/api/@tanstack/react-query.gen";
import { apiClient, queryClient } from "@/client";
import { useQuery } from "@tanstack/react-query";
import { DataTable, type DataTableRowClickEvent } from "primereact/datatable";
import { Column } from "primereact/column";
import type { StopView, TrainView } from "@/api";
import { displayDate } from "@/utils/date";
import { useMemo } from "react";
import { arrivalComparer, displayTime } from "@/utils/stop";

export const Route = createFileRoute("/stations/$ds100/$date")({
  loader: async ({ params }) => {
    await queryClient.ensureQueryData({
      ...stationOptions({
        client: apiClient,
        path: {
          ds100: params.ds100,
        },
      }),
    });

    await queryClient.ensureQueryData({
      ...trainsForStationOptions({
        client: apiClient,
        path: {
          ds100: params.ds100,
          date: params.date,
        },
      }),
    });

    await queryClient.ensureQueryData({
      ...stopsForStationOptions({
        client: apiClient,
        path: {
          ds100: params.ds100,
          date: params.date,
        },
      }),
    });
  },
  component: RouteComponent,
});

function RouteComponent() {
  const { ds100, date } = Route.useParams();

  const router = useRouter();

  const { data: station, isSuccess: isSuccessStation } = useQuery({
    ...stationOptions({
      client: apiClient,
      path: {
        ds100: ds100,
      },
    }),
  });

  const { data: trains, isSuccess: isSuccessTrains } = useQuery({
    ...trainsForStationOptions({
      client: apiClient,
      path: {
        ds100: ds100,
        date: date,
      },
    }),
  });

  const { data: stops, isSuccess: isSuccessStops } = useQuery({
    ...stopsForStationOptions({
      client: apiClient,
      path: {
        ds100: ds100,
        date: date,
      },
    }),
  });

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

  if (!isSuccessTrains && !isSuccessStation && !isSuccessStops) {
    return <div>Loading...</div>;
  }
  console.log(stopsByTrain);

  const handleRowClick = (e: DataTableRowClickEvent) => {
    const train = e.data as TrainView;
    router.navigate({ to: `/trains/${train.id}` });
  };

  return (
    <div>
      <div className="flex justify-between mb-4">
        <h2 className="text-2xl font-bold inline">
          {station?.name} ({station?.ds100})
        </h2>
        <h2 className="text-2xl font-bold inline">{displayDate(date)}</h2>
      </div>
      <h3 className="text-xl font-bold my-3">Züge</h3>
      <DataTable
        onRowClick={handleRowClick}
        size="small"
        value={trainsSorted}
        tableStyle={{ minWidth: "50rem" }}
        rowHover
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
    </div>
  );
}
