import {
  Link,
  Outlet,
  createRootRoute,
  useRouter,
} from "@tanstack/react-router";
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
      label: "Stations",
      icon: "pi pi-fw pi-map-marker",
      command: () => {
        router.navigate({ to: "/stations" });
      },
    },
  ];

  return (
    <>
      <Menubar model={items} />
      <hr />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
};

export const Route = createRootRoute({ component: RootLayout });
