import type { MovementView, StopView } from "@/api";
import { getTimestamp } from "./date";

export function getPlatform(stop: StopView): string {
  return stop.arrival?.platform ?? stop.departure?.platform ?? "Unbekannt";
}

export function getArrivalTS(stop: StopView): number | null | undefined {
  return getTimestamp(stop.arrival?.planned);
}

export function getDepartureTS(stop: StopView): number | null | undefined {
  return getTimestamp(stop.departure?.planned);
}

export function arrivalComparer(a: StopView, b: StopView): number {
  const tsA = getArrivalTS(a) ?? getDepartureTS(a) ?? 0;
  const tsB = getArrivalTS(b) ?? getDepartureTS(b) ?? 0;

  return tsA - tsB;
}

export function departureComparer(a: StopView, b: StopView) {
  const tsA = getDepartureTS(a) ?? getArrivalTS(a) ?? 0;
  const tsB = getDepartureTS(b) ?? getArrivalTS(b) ?? 0;

  return tsA - tsB;
}

export function sortByArrival(stops: StopView[]) {
  stops.sort(departureComparer);
}

export function sortByDeparture(stops: StopView[]) {
  stops.sort(arrivalComparer);
}

export function displayTime(mov?: MovementView | null): string {
  if (!mov || !mov.planned) return "";

  const date = new Date(mov.planned);
  return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}
