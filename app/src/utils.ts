export function formatDateYYMMDD(date: Date): string {
  const year = date.getFullYear().toString().slice(-2); // last 2 digits
  const month = String(date.getMonth() + 1).padStart(2, "0"); // months are 0-based
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}${month}${day}`;
}
