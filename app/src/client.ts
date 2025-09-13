import { QueryClient } from "@tanstack/react-query";
import { createClient } from "./api/client";

export const apiClient = createClient({
  baseUrl: "https://db-iris.it-solutions-mayer.de/v1",
});

export const queryClient = new QueryClient();
