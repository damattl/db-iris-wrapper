import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  input: "https://db-iris.it-solutions-mayer.de/v1/openapi.json",
  output: "src/api",
  plugins: ["@hey-api/client-fetch", "@tanstack/react-query"],
});
