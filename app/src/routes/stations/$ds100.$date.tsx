import { createFileRoute } from "@tanstack/react-router";
import {
  stationOptions,
  stopsForStationOptions,
  trainsForStationOptions,
} from "@/api/@tanstack/react-query.gen";
import { apiClient, queryClient } from "@/client";
import { useQuery } from "@tanstack/react-query";
import { displayDate } from "@/utils/date";
import { TrainViewTable } from "@/components/train_view_table";

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

  if (!isSuccessTrains && !isSuccessStation && !isSuccessStops) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div className="flex justify-between mb-4">
        <h2 className="text-2xl font-bold inline">
          {station?.name} ({station?.ds100} / {station?.id})
        </h2>
        <h2 className="text-2xl font-bold inline">{displayDate(date)}</h2>
      </div>
      <h3 className="text-xl font-bold my-3">Züge</h3>
      <TrainViewTable
        station={station}
        stops={stops ?? []}
        trains={trains ?? []}
      />
    </div>
  );
}
