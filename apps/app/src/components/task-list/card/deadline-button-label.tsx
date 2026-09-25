import { StyleSheet } from 'react-native';
import Animated from 'react-native-reanimated';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import dayjs from '@lib/datetime';

import Button from '@components/primitives/button';
import Icon from '@components/primitives/icon';

type DeadlineButtonLabelProps = {
  date?: dayjs.Dayjs;
  disabled?: boolean;
  onPress?: () => void;
};

export default function DeadlineButtonLabel({
  disabled = false,
  ...props
}: DeadlineButtonLabelProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return (
    <Animated.View style={styles.container}>
      <Button
        scheme="tertiary"
        typographyStyle={styles.text}
        icon={
          <Icon
            name="flag"
            size={22}
            color={theme.palette.danger}
          />
        }
        onPress={() => {
          props.onPress?.();
        }}
        disabled={disabled}
      >
        {props.date && formatDate(props.date)}
      </Button>
    </Animated.View>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      flexDirection: 'row',
      alignItems: 'center',
    },
    text: {
      ...theme.components.Typography.variants.text,
    },
  });
}

function formatDate(inputDate: dayjs.Dayjs) {
  const today = dayjs().startOf('day');
  const nextYear = today.startOf('year').add(1, 'year');

  const date = inputDate.startOf('day');

  if (date.isSame(today)) {
    // today
    return 'Today';
  } else if (date.isBefore(today)) {
    // before today
    return date.format(`MM/DD ${today.diff(date, 'day')}[d ago]`);
  } else if (date.isBefore(nextYear)) {
    // within the year
    return date.format(`ddd, MMM D ${date.diff(today, 'day')}[d left]`);
  } else {
    return date.format('ddd, MM/YYYY');
  }
}
