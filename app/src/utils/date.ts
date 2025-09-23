/** Returns the stored time value in milliseconds since midnight, January 1, 1970 UTC. */
export function getTimestamp(
  date?: Date | string | number | null,
): number | null | undefined {
  return parseDate(date)?.getTime();
}

/** Returns the stored time value in milliseconds since midnight, January 1, 1970 UTC. */
export function getTimestampNotNull(
  date?: Date | string | number | null,
): number {
  return getTimestamp(date) ?? 0;
}

export function parseDate(
  date?: Date | string | number | null,
): Date | null | undefined {
  if (!date) {
    return null;
  }

  if (date instanceof Date) {
    return date;
  }
  if (typeof date == "string" && date.length == 6) {
    const str = `20${date.substring(0, 2)}-${date.substring(2, 4)}-${date.substring(4)}`;
    return new Date(str);
  }
  if (typeof date == "number" || typeof date == "string") {
    return new Date(date);
  }
  return null;
}

export function displayDate(
  date?: Date | string | number | null,
): string | null | undefined {
  if (!date) {
    return null;
  }

  const options: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  };

  return parseDate(date)?.toLocaleDateString("de-DE", options);
}

export function displayDateTime(
  date?: Date | string | number | null,
): string | null | undefined {
  if (!date) {
    return null;
  }

  const options: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  };

  return parseDate(date)?.toLocaleDateString("de-DE", options);
}
