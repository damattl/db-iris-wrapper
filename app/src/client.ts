import { createClient } from "./api/client";

export const defaultClient = createClient({
  baseUrl: "https://db-iris.it-solutions-mayer.de/v1",
});
