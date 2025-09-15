import { toastRefAtom } from "@/atoms";
import { switchTheme } from "@/theme";
import { Outlet, createRootRoute, useRouter } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { useAtom } from "jotai";
import { Button } from "primereact/button";
import { Menubar } from "primereact/menubar";
import { Toast } from "primereact/toast";
import { useState } from "react";

const RootLayout = () => {
  const router = useRouter();
  const [toastRef] = useAtom(toastRefAtom);

  const [dark, setDark] = useState(false);

  const toggleTheme = () => {
    if (dark) {
      switchTheme("saga-blue");
    } else {
      switchTheme("arya-blue");
    }
    setDark(!dark);
  };

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
    {
      label: "Züge",
      icon: "pi pi-fw pi-database",
      command: () => {
        router.navigate({ to: "/trains" });
      },
    },
    {
      label: "Meldungen",
      icon: "pi pi-fw pi-envelope",
      command: () => {
        router.navigate({ to: "/messages" });
      },
    },
  ];

  return (
    <div className="m-2 mb-20">
      <Menubar
        className="mb-2"
        model={items}
        end={
          <Button
            icon={dark ? "pi pi-fw pi-sun" : "pi pi-fw pi-moon"}
            onClick={toggleTheme}
            rounded
            text
            severity="secondary"
            size="small"
          />
        }
      />
      <Outlet />
      <TanStackRouterDevtools />
      <Toast ref={toastRef} />
    </div>
  );
};

export const Route = createRootRoute({ component: RootLayout });
