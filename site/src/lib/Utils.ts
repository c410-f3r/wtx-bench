import type Csv from './Csv';

const ONE_DAY = 1000 * 60 * 60 * 24;

export const MAX_DAYS = 14;

export function dateAndTime(date: Date): string {
  return date.toLocaleString();
}

export function environmentStateParams(
  csv: Csv,
  environment: string
): {
  lastDays: number;
  protocol: string;
} {
  const { count } = oldestFromEnvironment(csv, environment);
  return {
    lastDays: Math.min(count, MAX_DAYS),
    protocol: protocolState()
  };
}

export function firstDateOfLastDays(lastDays: number): Date {
  const date = new Date();
  date.setDate(date.getDate() - lastDays);
  date.setHours(24, 0, 0, 0);
  return date;
}

export function protocolState(): string {
  return "web-socket";
}

export function randomColor() {
  const letters = '0123456789ABCDEF'.split('');
  let color = '#';
  for (let i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

function oldestFromEnvironment(csv: Csv, environment: string): { count: number; date: number } {
  const iter = csv.results.get(environment)!.keys();
  const firstDate = iter.next().value;
  const now = firstDateOfLastDays(0);
  const countDiffFn = (date: number) => Math.ceil((now.getTime() - date) / ONE_DAY);

  let count = countDiffFn(firstDate);
  let date = firstDate;
  for (const localDate of iter) {
    const countDiff = countDiffFn(localDate);
    if (countDiff > count) {
      count = countDiff;
      date = localDate;
    }
  }

  return { count, date };
}
