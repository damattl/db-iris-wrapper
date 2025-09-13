import type { TrainView } from "@/api";
import { sortStopsByTime } from "./stop";

export function getStartToEnd(train: TrainView) {
  const stops = [...train.next_stops, ...train.past_stops];
  sortStopsByTime(stops);

  if (stops.length < 2) {
    return "";
  }

  const first = stops[0];
  const last = stops[stops.length - 1];
  return `${first.station?.name} -> ${last.station?.name}`;
}

export function getFullTrainName(
  train: TrainView,
  includeDate: boolean = false,
): string {
  let name = train.category;
  if (train.line) {
    name += ` ${train.line} - ${train.number}`;
  } else {
    name += ` ${train.number}`;
  }
  if (includeDate) {
    const date = new Date(train.date);
    name += ` @${date.toLocaleDateString("de-DE")}`;
  }
  return name;
}
