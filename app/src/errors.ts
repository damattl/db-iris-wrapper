import { useAtom } from "jotai";
import { toastRefAtom } from "./atoms";

export const useShowError = () => {
  const [toastRef] = useAtom(toastRefAtom);

  return (summary: string, detail?: string) => {
    toastRef.current?.show({
      summary: summary,
      detail: detail,
      severity: "error",
    });
  };
};
