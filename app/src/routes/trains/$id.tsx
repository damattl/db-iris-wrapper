import {
  messagesForTrainOptions,
  statusCodesOptions,
  trainByIdOptions,
} from "@/api/@tanstack/react-query.gen";
import { apiClient } from "@/client";
import { MessageViewTable } from "@/components/message_view_table";
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

  const { data: codes } = useQuery({
    ...statusCodesOptions({
      client: apiClient,
    }),
    refetchOnWindowFocus: false,
    refetchOnMount: false,
  });

  const { data: train, isSuccess: isSuccessTrain } = useQuery({
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

  const { data: messages, isSuccess: isSuccessMessages } = useQuery({
    ...messagesForTrainOptions({
      client: apiClient,
      path: {
        train_id: id,
      },
    }),
  });

  if (!isSuccessMessages || !isSuccessTrain) {
    return <div>Lädt...</div>;
  }

  console.log(messages);

  return (
    <div>
      <h2 className="text-2xl font-bold mb-2">{getFullTrainName(train)}</h2>
      <h3>{displayDate(train?.date)}</h3>
      <span>{getStartToEnd(train)}</span>

      <h3 className="text-xl font-bold my-3">Zukünftige Stops</h3>
      <StopViewTable stops={train.next_stops} nextStop={train.next_stop} />

      <h3 className="text-xl font-bold mt-6 my-3">Vergangene Stops</h3>
      <StopViewTable stops={train.past_stops} nextStop={train.next_stop} />

      <h3 className="text-xl font-bold mt-6 my-3">Meldungen</h3>
      <MessageViewTable
        codes={codes ?? []}
        messages={messages.filter((m) => m.code)}
      />
    </div>
  );

  return <div>Hello "/trains/$id"!</div>;
}
