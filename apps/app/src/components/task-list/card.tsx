import { Pressable, StyleSheet, View } from 'react-native';
import Animated, { FadeIn, FadeOut } from 'react-native-reanimated';

import { task } from '@clearlist/types';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Icon from '@/components/icon';
import Button from '@/components/primitives/button';
import Slot from '@/components/primitives/slot';
import EditableTypography from '@/components/text/editable-typography';

type TaskCardProps = {
  task: task.Task;
  expanded?: boolean;
  onToggle?: () => void;
  onTaskUpdate?: (patch: Partial<task.UpdateRequest>) => void;
  onTaskComplete?: () => void;
  onTaskReopen?: () => void;
  onPressStartDate?: () => void;
  onPressDeadline?: () => void;
  onPressTags?: () => void;
};

export default function TaskCard({
  task,
  expanded,
  onToggle,
  ...props
}: TaskCardProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return (
    <Pressable
      style={[styles.container, expanded ? styles.expandedContainer : null]}
      onPress={() => onToggle?.()}
    >
      <Animated.View
        style={styles.mainRow}
        entering={FadeIn}
        exiting={FadeOut}
      >
        <Pressable
          onPress={task.completedAt ? props.onTaskReopen : props.onTaskComplete}
        >
          {task.completedAt ? (
            <Icon name="checkbox" />
          ) : (
            <Icon name="square-outline" />
          )}
        </Pressable>

        <EditableTypography
          value={task.title}
          onSave={(title) => {
            props.onTaskUpdate?.({ title: title });
          }}
          placeholder="New Task"
          disabled={!expanded}
          style={styles.titleTypography}
          containerStyle={styles.titleTypography}
        />
      </Animated.View>

      {expanded && (
        <>
          <View style={styles.fieldPanel}>
            <Slot />

            <View style={styles.inputPanel}>
              <EditableTypography
                value={task.notes || undefined}
                onSave={(notes) => {
                  props.onTaskUpdate?.({ notes: notes });
                }}
                placeholder="Notes"
                multiline
              />
            </View>
          </View>

          <View style={styles.buttonSet}>
            <Button
              icon={
                <Icon
                  name="calendar-outline"
                  size={22}
                />
              }
              onPress={props.onPressStartDate}
            />
            <Button
              icon={
                <Icon
                  name="flag-outline"
                  size={22}
                />
              }
              onPress={props.onPressDeadline}
            />
            <Button
              icon={
                <Icon
                  name="pricetag-outline"
                  size={22}
                />
              }
            />
          </View>
        </>
      )}
    </Pressable>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      borderRadius: theme.rounded.lg,
      padding: theme.spacings.x3,

      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      alignItems: 'stretch',
      gap: theme.spacings.x4,
    },
    expandedContainer: {
      backgroundColor: theme.palette.surface,
    },
    mainRow: {
      flex: 1,

      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-start',
      alignItems: 'center',
      gap: theme.spacings.x4,
    },
    titleTypography: {
      flex: 1,

      fontSize: 20,
    },
    fieldPanel: {
      display: 'flex',
      flexDirection: 'row',
      gap: theme.spacings.x4,
    },
    inputPanel: {
      flex: 1,

      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'flex-start',
      alignItems: 'stretch',
      gap: theme.spacings.x4,
    },
    buttonSet: {
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-end',
      alignItems: 'center',
      gap: theme.spacings.x4,
    },
  });
}
