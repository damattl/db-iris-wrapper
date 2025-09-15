import { QueryClient } from "@tanstack/react-query";
import { createClient } from "./api/client";

export const apiClient = createClient({
  baseUrl: import.meta.env.VITE_API_BASE_URL,
});

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      retry: (failureCount, error: any) => {
        if (error?.code == 404) {
          return false; // do not retry on 404
        }
        return failureCount < 3; // otherwise retry up to 3 times
      },
    },
  },
});
