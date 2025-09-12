import { useQuery } from "@tanstack/react-query";
import { createFileRoute, useRouter } from "@tanstack/react-router";
import { stationsOptions } from "../api/@tanstack/react-query.gen";
import { defaultClient } from "../client";
import { DataTable, type DataTableRowClickEvent } from "primereact/datatable";
import { Column } from "primereact/column";
import type { StationView } from "../api";
import { formatDateYYMMDD } from "../utils";

export const Route = createFileRoute("/stations")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data, isSuccess } = useQuery({
    ...stationsOptions({
      client: defaultClient,
    }),
  });

  const router = useRouter();

  if (!isSuccess) {
    return <div>Loading...</div>;
  }

  const handleRowClick = (e: DataTableRowClickEvent) => {
    const station = e.data as StationView;
    const today = new Date();
    const date = formatDateYYMMDD(today);

    router.navigate({ to: `/trains/${station.ds100}/${date}` });
  };

  return (
    <DataTable
      onRowClick={handleRowClick}
      size="small"
      value={data}
      tableStyle={{ minWidth: "50rem" }}
    >
      <Column field="id" header="ID"></Column>
      <Column field="name" header="Name"></Column>
      <Column field="ds100" header="DS1000"></Column>
      <Column field="lat" header="Latitude"></Column>
      <Column field="lon" header="Longitude"></Column>
    </DataTable>
  );
}
