import { Outlet, createRootRoute, useRouter } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { Menubar } from "primereact/menubar";

const RootLayout = () => {
  const router = useRouter();
  const items = [
    {
      label: "Home",
      icon: "pi pi-fw pi-home",
      command: () => {
        router.navigate({ to: "/" });
      },
    },
    {
      label: "Bahnhöfe",
      icon: "pi pi-fw pi-map-marker",
      command: () => {
        router.navigate({ to: "/stations" });
      },
    },
  ];

  return (
    <div className="m-2">
      <Menubar className="mb-2" model={items} />
      <Outlet />
      <TanStackRouterDevtools />
    </div>
  );
};

export const Route = createRootRoute({ component: RootLayout });
