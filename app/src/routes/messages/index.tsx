import type { StatusCodeView } from "@/api";
import {
  messagesForDateAndCodeOptions,
  statusCodesOptions,
} from "@/api/@tanstack/react-query.gen";
import { toastRefAtom } from "@/atoms";
import { apiClient } from "@/client";
import { MessageViewTable } from "@/components/message_view_table";
import { formatDateYYMMDD } from "@/utils";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { useAtom } from "jotai";
import { Button } from "primereact/button";
import { Calendar } from "primereact/calendar";
import { Dropdown } from "primereact/dropdown";
import { useState } from "react";

export const Route = createFileRoute("/messages/")({
  component: RouteComponent,
});

function RouteComponent() {
  const [date, setDate] = useState<Date | null>(new Date());
  const [code, setCode] = useState<StatusCodeView | null>(null);

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
    refetchOnWindowFocus: false,
    queryFn: async (params) => {
      const result = await queryOptions.queryFn!(params);
      toastRef.current?.show({
        summary: `${result.length} Meldungen gefunden`,
        severity: "info",
      });
      return result;
    },
  });

  const { data: codes } = useQuery({
    ...statusCodesOptions({
      client: apiClient,
    }),
    refetchOnWindowFocus: false,
    refetchOnMount: false,
  });

  if (query.isLoading) return <p>Loading...</p>;
  if (query.isError) return <p>Error: {(query.error as Error).message}</p>;

  const handleSearch = () => {
    if (date && code) {
      setQueryParams({
        date: date,
        code: code.code,
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

        <Dropdown
          value={code}
          onChange={(e) => setCode(e.value)}
          options={codes?.filter((code) => code.code != 0)}
          optionLabel="long_text"
          editable
          className="md:w-auto w-full"
          placeholder="Status"
        />

        <Button
          label="Suchen"
          icon="pi pi-search"
          onClick={handleSearch}
          disabled={isDisabled}
        />
      </div>

      <MessageViewTable
        codes={codes ?? []}
        messages={query.data ?? []}
      ></MessageViewTable>
    </div>
  );
}
