import { createFileRoute } from "@tanstack/react-router";
import { trainsForStationOptions } from "../api/@tanstack/react-query.gen";
import { defaultClient } from "../client";
import { useQuery } from "@tanstack/react-query";
import { DataTable } from "primereact/datatable";
import { Column } from "primereact/column";

export const Route = createFileRoute("/trains/$ds100/$date")({
  component: RouteComponent,
});

function RouteComponent() {
  const { ds100, date } = Route.useParams();

  const { data, isSuccess } = useQuery({
    ...trainsForStationOptions({
      client: defaultClient,
      path: {
        ds100: ds100,
        date: date,
      },
    }),
  });

  if (!isSuccess) {
    return <div>Loading...</div>;
  }

  return (
    <DataTable size="small" value={data} tableStyle={{ minWidth: "50rem" }}>
      <Column field="id" header="ID"></Column>
      <Column field="operator" header="Operator"></Column>
      <Column field="category" header="Category"></Column>
      <Column field="line" header="Line"></Column>
      <Column field="date" header="Date"></Column>
    </DataTable>
  );
}
