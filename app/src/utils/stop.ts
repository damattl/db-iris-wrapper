import type { MovementView, StopView } from "@/api";

export function getPlatform(stop: StopView): string {
  return stop.arrival?.platform ?? stop.departure?.platform ?? "Unbekannt";
}

export function getArrivalTS(stop: StopView): number | null {
  return stop.arrival?.planned
    ? new Date(stop.arrival?.planned).getTime()
    : null;
}

export function getStopTS(stop: StopView): number | null {
  return stop.arrival?.planned
    ? new Date(stop.arrival?.planned).getTime()
    : null;
}

export function sortStopsByTime(stops: StopView[]) {
  stops.sort((a, b) => {
    if ((getArrivalTS(a) ?? 0) < (getArrivalTS(b) ?? 0)) {
      return -1;
    }
    return 1;
  });
}

export function displayTime(mov?: MovementView | null): string {
  if (!mov || !mov.planned) return "";

  const date = new Date(mov.planned);
  return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}
