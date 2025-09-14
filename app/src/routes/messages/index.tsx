import { messagesForDateAndCodeOptions } from "@/api/@tanstack/react-query.gen";
import { toastRefAtom } from "@/atoms";
import { apiClient } from "@/client";
import { MessageViewTable } from "@/components/message_view_table";
import { formatDateYYMMDD } from "@/utils";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { useAtom } from "jotai";
import { Button } from "primereact/button";
import { Calendar } from "primereact/calendar";
import { InputNumber } from "primereact/inputnumber";
import { useState } from "react";

export const Route = createFileRoute("/messages/")({
  component: RouteComponent,
});

function RouteComponent() {
  const [date, setDate] = useState<Date | null>(null);
  const [code, setCode] = useState<number | null>(null);

  const [queryParams, setQueryParams] = useState<{
    date: Date;
    code: number;
  } | null>(null);

  const [toastRef] = useAtom(toastRefAtom);

  const queryOptions = messagesForDateAndCodeOptions({
    client: apiClient,
    path: {
      date: formatDateYYMMDD(queryParams?.date ?? new Date()),
      code: queryParams?.code ?? 0,
    },
  });

  const query = useQuery({
    ...queryOptions,
    enabled: queryParams != null,
    queryFn: async (params) => {
      const result = await queryOptions.queryFn!(params);
      toastRef.current?.show({
        summary: `${result.length} Meldungen gefunden`,
        severity: "info",
      });
      return result;
    },
  });

  if (query.isLoading) return <p>Loading...</p>;
  if (query.isError) return <p>Error: {(query.error as Error).message}</p>;

  const handleSearch = () => {
    if (date && code) {
      setQueryParams({
        date: date,
        code: code,
      });
      console.debug("Searching with:", { date, code });
    }
  };

  const isDisabled = !date || !code;

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

        <InputNumber
          value={code}
          onChange={(e) => setCode(e.value)}
          min={1}
          max={100}
          className="md:w-auto w-full"
          placeholder="Code"
        />

        <Button
          label="Suchen"
          icon="pi pi-search"
          onClick={handleSearch}
          disabled={isDisabled}
        />
      </div>

      <MessageViewTable messages={query.data ?? []}></MessageViewTable>
    </div>
  );
}
