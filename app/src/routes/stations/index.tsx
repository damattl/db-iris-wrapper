import { useQuery } from "@tanstack/react-query";
import { createFileRoute, useRouter } from "@tanstack/react-router";
import { stationsOptions } from "../../api/@tanstack/react-query.gen";
import { apiClient } from "../../client";
import {
  DataTable,
  type DataTableFilterMeta,
  type DataTableFilterMetaData,
  type DataTableRowClickEvent,
} from "primereact/datatable";
import { Column } from "primereact/column";
import type { StationView } from "../../api";
import { formatDateYYMMDD } from "../../utils";
import { useState, type ChangeEvent } from "react";
import { FilterMatchMode } from "primereact/api";
import { InputText } from "primereact/inputtext";
import { InputIcon } from "primereact/inputicon";
import { IconField } from "primereact/iconfield";

export const Route = createFileRoute("/stations/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data, isSuccess } = useQuery({
    ...stationsOptions({
      client: apiClient,
    }),
  });

  const router = useRouter();

  const [filters, setFilters] = useState<DataTableFilterMeta>({
    global: { value: null, matchMode: FilterMatchMode.CONTAINS },
  });
  const [globalFilterValue, setGlobalFilterValue] = useState("");

  if (!isSuccess) {
    return <div>Lädt...</div>;
  }

  const handleRowClick = (e: DataTableRowClickEvent) => {
    const station = e.data as StationView;
    const today = new Date();
    const date = formatDateYYMMDD(today);

    router.navigate({ to: `/stations/${station.ds100}/${date}` });
  };

  const onGlobalFilterChange = (e: ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    const _filters = { ...filters };

    (_filters["global"] as DataTableFilterMetaData).value = value;

    setFilters(_filters);
    setGlobalFilterValue(value);
  };

  const renderHeader = () => {
    return (
      <div className="flex justify-content-end">
        <IconField iconPosition="left">
          <InputIcon className="pi pi-search" />
          <InputText
            value={globalFilterValue}
            onChange={onGlobalFilterChange}
            placeholder="Suche"
          />
        </IconField>
      </div>
    );
  };

  const header = renderHeader();

  return (
    <DataTable
      onRowClick={handleRowClick}
      filters={filters}
      globalFilterFields={["name", "ds100"]}
      size="small"
      value={data}
      tableStyle={{ minWidth: "50rem" }}
      header={header}
      rowHover
    >
      <Column field="id" header="ID"></Column>
      <Column field="name" header="Name"></Column>
      <Column field="ds100" header="DS1000"></Column>
      <Column field="lat" header="Latitude"></Column>
      <Column field="lon" header="Longitude"></Column>
    </DataTable>
  );
}
