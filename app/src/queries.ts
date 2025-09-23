import { useQuery } from "@tanstack/react-query";
import {
  stationsOptions,
  statusCodesOptions,
} from "./api/@tanstack/react-query.gen";
import { apiClient } from "./client";

const ONE_MONTH = 1000 * 60 * 60 * 24 * 30;

export function useStations() {
  return useQuery({
    ...stationsOptions({
      client: apiClient,
    }),
    staleTime: ONE_MONTH,
    refetchOnMount: false,
    refetchOnReconnect: false,
    refetchOnWindowFocus: false,
  });
}

export function useStatusCodes() {
  return useQuery({
    ...statusCodesOptions({
      client: apiClient,
    }),
    staleTime: ONE_MONTH,
    refetchOnMount: false,
    refetchOnReconnect: false,
    refetchOnWindowFocus: false,
  });
}
