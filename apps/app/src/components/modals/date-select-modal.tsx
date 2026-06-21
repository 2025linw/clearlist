import dayjs from 'dayjs';

import { Modal, Pressable, StyleSheet } from 'react-native';
import DateTimePicker, { useDefaultStyles } from 'react-native-ui-datepicker';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';
import { getTodayDate, toDate } from '@/services/helpers';

type Props = {
  visible?: boolean;
  initialDate?: string;
  onDateSelect?: (date: Date) => void;
  dismiss?: () => void;
};

export default function DateSelectModal({ initialDate, ...props }: Props) {
  const theme = useTheme();
  const styles = buildStyles(theme);
  const defaultStyles = useDefaultStyles();

  return (
    <Modal
      visible={props.visible}
      transparent
    >
      <Pressable
        onPress={props.dismiss}
        style={styles.overlay}
      >
        <Pressable
          onPress={(e) => e.stopPropagation()}
          style={styles.container}
        >
          <DateTimePicker
            mode="single"
            date={dayjs(initialDate)}
            onChange={({ date }) => {
              const jsDate = toDate(date);
              if (!jsDate) return;

              props.onDateSelect?.(jsDate);
            }}
            minDate={getTodayDate()}
            styles={defaultStyles}
          />
        </Pressable>
      </Pressable>
    </Modal>
  );
}

function buildStyles(theme: Theme) {
  const styles = StyleSheet.create({
    overlay: {
      flex: 1,

      paddingHorizontal: theme.spacings.xl,

      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      alignItems: 'center',
    },
    container: {
      borderRadius: theme.rounded.lg,
      padding: theme.spacings.lg,

      backgroundColor: theme.palette.surface,
    },
  });

  return styles;
}
