import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import dayjs, { getDayOfWeek } from '@lib/datetime';

import Icon from '@components/primitives/icon';
import Typography from '@components/primitives/typography';

type StartBadgeProps = {
  date: dayjs.Dayjs;
};

export default function StartBadge(props: StartBadgeProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  const today = dayjs().startOf('day');
  const date = props.date.startOf('day');

  if (date.isSame(today)) {
    // today
    return (
      <Icon
        name="star"
        color="gold"
      />
    );
  } else {
    return (
      <Typography style={styles.text}>{formatDate(props.date)}</Typography>
    );
  }
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
  const nextWeek = today.add(1, 'week');
  const nextYear = today.startOf('year').add(1, 'year');

  const date = inputDate.startOf('day');

  if (date.isSame(today)) {
    // today
    return 'Today';
  } else if (date.isBefore(today)) {
    // before today
    return date.format('MM/DD');
  } else if (date.isBefore(nextWeek)) {
    // within the next week
    return getDayOfWeek(date, true);
  } else if (date.isBefore(nextYear)) {
    // within the next year
    return date.format('MM/DD');
  } else {
    return date.format('MM/YYYY');
  }
}
