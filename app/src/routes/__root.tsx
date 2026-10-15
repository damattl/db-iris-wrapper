import { toastRefAtom } from "@/atoms";
import { switchTheme } from "@/theme";
import { Outlet, createRootRoute, useRouter } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { useAtom } from "jotai";
import { Button } from "@mantle-ui/react/button";
import { Menubar } from "@mantle-ui/react/menubar";
import { Toast } from "@mantle-ui/react/toast";
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
      icon: "mi mi-fw mi-home",
      command: () => {
        router.navigate({ to: "/" });
      },
    },
    {
      label: "Bahnhöfe",
      icon: "mi mi-fw mi-map-marker",
      command: () => {
        router.navigate({ to: "/stations" });
      },
    },
    {
      label: "Züge",
      icon: "mi mi-fw mi-database",
      command: () => {
        router.navigate({ to: "/trains" });
      },
    },
    {
      label: "Meldungen",
      icon: "mi mi-fw mi-envelope",
      command: () => {
        router.navigate({ to: "/messages" });
      },
    },
  ];

  return (
    <div className="my-2 mx-4 mb-20">
      <Menubar
        className="mb-2"
        model={items}
        end={
          <Button
            icon={dark ? "mi mi-fw mi-sun" : "mi mi-fw mi-moon"}
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
