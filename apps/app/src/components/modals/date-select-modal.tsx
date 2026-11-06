import { Modal, Pressable, StyleSheet, View } from 'react-native';
import DateTimePicker from 'react-native-ui-datepicker';

import dayjs, { getDateToday } from '@lib/datetime';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import Button from '@components/primitive/button';
import Icon from '@components/primitive/icon';
import Typography from '@components/primitive/typography';

export type Mode = 'start' | 'deadline';

type DateSelectModalProps = {
  visible?: boolean;
  mode: Mode;
  taskId: string;
  initialDate?: dayjs.Dayjs;
  onDateSelect?: (selectedDate: dayjs.Dayjs | null) => void;
  dismiss?: () => void;
};

export default function DateSelectModal({
  mode,
  taskId,
  initialDate,
  ...props
}: DateSelectModalProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return (
    <Modal
      visible={props.visible}
      transparent
    >
      <View style={styles.container}>
        <Pressable
          onPress={props.dismiss}
          style={StyleSheet.absoluteFill}
        />

        <View style={styles.content}>
          <View style={styles.header}>
            <Button
              scheme="danger"
              leftIcon={<Icon name="trash-bin" />}
              style={styles.headerButton}
              onPress={() => props.onDateSelect?.(null)}
            />

            <Typography>{mode === 'start' ? 'When?' : 'Deadline'}</Typography>

            <Button
              scheme="secondary"
              hasBorder={false}
              leftIcon={<Icon name="close" />}
              style={styles.headerButton}
              onPress={props.dismiss}
            />
          </View>

          <DateTimePicker
            mode="single"
            date={initialDate ? dayjs(initialDate) : undefined}
            onChange={({ date: date_type }) => {
              const date = dayjs(date_type);
              if (!date.isValid()) return;

              props.onDateSelect?.(date);
            }}
            minDate={getDateToday()}
            styles={{
              button_prev_image: { tintColor: theme.palette.primary },
              button_next_image: { tintColor: theme.palette.primary },

              disabled_label: { color: theme.palette.subtle },

              month_label: { color: theme.palette.text },
              month_selector_label: { color: theme.palette.primary },
              selected_month_label: { color: theme.palette.primary },

              year_label: { color: theme.palette.text },
              year_selector_label: { color: theme.palette.primary },
              selected_year_label: { color: theme.palette.primary },

              weekday_label: { color: theme.palette.primary },

              day_label: { color: theme.palette.text },
              today_label: { color: theme.palette.danger },
              selected_label: { color: theme.palette.success },
            }}
          />

          <Button>Set Time</Button>
        </View>
      </View>
    </Modal>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      flex: 1,

      paddingHorizontal: theme.spacings.x6,

      justifyContent: 'center',
      alignItems: 'center',
    },
    content: {
      gap: theme.spacings.x2,

      borderWidth: 1,
      borderColor: theme.palette.border,
      borderRadius: theme.rounded.lg,
      padding: theme.spacings.x4,

      backgroundColor: theme.palette.surface,
    },
    header: {
      flexDirection: 'row',
      justifyContent: 'space-between',
    },
    headerButton: {
      borderRadius: theme.rounded.full,
    },
  });
}
