import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';
import { Theme } from '@contexts/theme/types';
import dayjs from '@lib/datetime';

import Typography from '@components/primitives/typography';

type DeadlineBadgeProps = {
  date: dayjs.Dayjs;
};

export default function DeadlineBadge(props: DeadlineBadgeProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return <Typography style={styles.text}>{formatDate(props.date)}</Typography>;
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    text: {
      color: theme.palette.primary,
    },
  });
}

function formatDate(inputDate: dayjs.Dayjs) {
  const today = dayjs().startOf('day');
  const nextMonth = today.add(1, 'month');
  const nextYear = today.add(1, 'year').startOf('year');

  const date = inputDate.startOf('day');

  if (date.isSame(today)) {
    // today
    return 'Today';
  } else if (date.isBefore(today)) {
    // before today
    return `${today.diff(date, 'day')}d ago`;
  } else if (date.isBefore(nextMonth)) {
    // within the next month
    return `${date.diff(today, 'day')}d left`;
  } else if (date.isBefore(nextYear)) {
    // within the next year
    return date.format('MM/DD');
  } else {
    return date.format('MM/YYYY');
  }
}
