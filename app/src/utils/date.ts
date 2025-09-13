export function displayDate(
  date?: Date | string | number | null,
): string | null {
  if (!date) {
    return null;
  }

  const options: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  };

  if (date instanceof Date) {
    return date.toLocaleDateString("de-DE", options);
  }
  if (typeof date == "string" && date.length == 6) {
    const str = `20${date.substring(0, 2)}-${date.substring(2, 4)}-${date.substring(4)}`;
    return new Date(str).toLocaleDateString("de-DE", options);
  }
  if (typeof date == "number" || typeof date == "string") {
    return new Date(date).toLocaleDateString("de-DE", options);
  }
  return "invalid date";
}
