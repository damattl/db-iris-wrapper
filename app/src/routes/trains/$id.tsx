import { trainByIdOptions } from "@/api/@tanstack/react-query.gen";
import { apiClient } from "@/client";
import { StopViewTable } from "@/components/stop_view_table";
import { displayDate } from "@/utils/date";
import { getFullTrainName, getStartToEnd } from "@/utils/train";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/trains/$id")({
  component: RouteComponent,
});

function RouteComponent() {
  const { id } = Route.useParams();

  const { data, isSuccess } = useQuery({
    ...trainByIdOptions({
      client: apiClient,
      path: {
        id: id,
      },
      query: {
        include_stops: true,
      },
    }),
  });

  if (!isSuccess) {
    return <div>Lädt...</div>;
  }

  console.log(data);

  return (
    <div>
      <h2 className="text-2xl font-bold mb-2">{getFullTrainName(data)}</h2>
      <h3>{displayDate(data?.date)}</h3>
      <span>{getStartToEnd(data)}</span>

      <h3 className="text-xl font-bold my-3">Zukünftige Stops</h3>
      <StopViewTable stops={data.next_stops} nextStop={data.next_stop} />

      <h3 className="text-xl font-bold mt-6 my-3">Vergangene Stops</h3>
      <StopViewTable stops={data.past_stops} nextStop={data.next_stop} />
    </div>
  );

  return <div>Hello "/trains/$id"!</div>;
}
