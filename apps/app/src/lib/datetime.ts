import dayjs from 'dayjs';
import isBetween from 'dayjs/plugin/isBetween';
import isSameOrAfter from 'dayjs/plugin/isSameOrAfter';
import utc from 'dayjs/plugin/utc';
import weekday from 'dayjs/plugin/weekday';

dayjs.extend(utc);
dayjs.extend(weekday);

// Comparisons
dayjs.extend(isBetween);
dayjs.extend(isSameOrAfter);

export function getDateToday() {
  return dayjs().startOf('day');
}

const weekDayMap: Record<number, string> = {
  0: 'Sunday',
  1: 'Monday',
  2: 'Tuesday',
  3: 'Wednesday',
  4: 'Thursday',
  5: 'Friday',
  6: 'Saturday',
} as const;

const weekDayShortMap: typeof weekDayMap = {
  0: 'Sun',
  1: 'Mon',
  2: 'Tue',
  3: 'Wed',
  4: 'Thur',
  5: 'Fri',
  6: 'Sat',
} as const;

export function getDayOfWeek(
  date: dayjs.Dayjs,
  short: boolean = false,
): string {
  // const { firstWeekday } = getCalendars()[0];
  const firstWeekday = null;

  const weekday = firstWeekday
    ? date.weekday(firstWeekday).weekday()
    : date.weekday();

  return short ? weekDayShortMap[weekday] : weekDayMap[weekday];
}

export default dayjs;
