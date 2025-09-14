import type { TrainError } from "@/api";
import { trainOptions } from "@/api/@tanstack/react-query.gen";
import { apiClient } from "@/client";
import { TrainViewTable } from "@/components/train_view_table";
import { useShowError } from "@/errors";
import { formatDateYYMMDD } from "@/utils";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { Button } from "primereact/button";
import { Calendar } from "primereact/calendar";
import { InputText } from "primereact/inputtext";
import { useState } from "react";

export const Route = createFileRoute("/trains/")({
  component: RouteComponent,
});

function RouteComponent() {
  const [date, setDate] = useState<Date | null>(null);
  const [number, setNumber] = useState<string | null>(null);

  const showError = useShowError();

  const [queryParams, setQueryParams] = useState<{
    date: Date;
    number: string;
  } | null>(null);

  const queryOptions = trainOptions({
    client: apiClient,
    path: {
      date: formatDateYYMMDD(queryParams?.date ?? new Date()),
      number: queryParams?.number ?? "",
    },
  });

  const query = useQuery({
    ...queryOptions,
    enabled: queryParams != null,
    queryFn: async (params) => {
      try {
        const result = await queryOptions.queryFn!(params);
        return result;
      } catch (e: unknown) {
        if ((e as TrainError).code == 404) {
          showError(`Zug nicht gefunden`);
        }
        throw e;
      }
    },
  });

  const handleSearch = () => {
    if (date && number) {
      setQueryParams({
        date: date,
        number: number,
      });
      console.debug("Searching with:", { date, number });
    }
  };

  const isDisabled = !date || !number;

  return (
    <div>
      <div className="flex flex-col gap-3 mb-10 md:flex-row">
        <Calendar
          value={date}
          onChange={(e) => setDate(e.value as Date)}
          placeholder="Datum"
          showIcon
          className="md:w-auto w-full"
          dateFormat="dd.mm.yy"
        />

        <InputText
          value={number ?? ""}
          onChange={(e) => setNumber(e.target.value)}
          className="md:w-auto w-full"
          placeholder="Nummer"
        />

        <Button
          label="Suchen"
          icon="pi pi-search"
          onClick={handleSearch}
          disabled={isDisabled}
          loading={query.isLoading}
        />
      </div>

      <TrainViewTable stops={[]} trains={query.data ? [query.data!] : []} />
    </div>
  );
}
