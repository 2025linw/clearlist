import { Pressable, StyleSheet, View } from 'react-native';

import { Task, TaskDTO } from '@clearlist/types';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Icon from '@/components/icon';
import Button from '@/components/primitives/button';
import Slot from '@/components/primitives/slot';
import EditableTypography from '@/components/text/editable-typography';

type TaskItemProp = {
  task: Task;
  expanded?: boolean;
  onToggle?: () => void;
  onTaskUpdate?: (patch: Partial<TaskDTO>) => void;
  onTaskComplete?: () => void;
  onTaskReopen?: () => void;
  onPressStartDate?: () => void;
  onPressDeadline?: () => void;
  onPressTags?: () => void;
};

export default function TaskItem({
  task,
  expanded,
  onToggle,
  ...props
}: TaskItemProp) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return (
    <Pressable
      style={[styles.container, expanded ? styles.expandedContainer : null]}
      onPress={() => onToggle?.()}
    >
      <View style={styles.mainRow}>
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
      </View>

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
              iconName="calendar-outline"
              iconSize={22}
              onPress={props.onPressStartDate}
            />
            <Button
              iconName="flag-outline"
              iconSize={22}
              onPress={props.onPressDeadline}
            />
            <Button
              iconName="pricetag-outline"
              iconSize={22}
            />
          </View>
        </>
      )}
    </Pressable>
  );
}

function buildStyles(theme: Theme) {
  const styles = StyleSheet.create({
    container: {
      borderRadius: theme.rounded.lg,
      padding: theme.spacings.xl,

      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      alignItems: 'stretch',
      gap: theme.spacings.lg,
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
      gap: theme.spacings.lg,
    },
    titleTypography: {
      flex: 1,

      fontSize: 20,
    },
    fieldPanel: {
      display: 'flex',
      flexDirection: 'row',
      gap: theme.spacings.lg,
    },
    inputPanel: {
      flex: 1,

      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'flex-start',
      alignItems: 'stretch',
      gap: theme.spacings.xl,
    },
    buttonSet: {
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-end',
      alignItems: 'center',
      gap: theme.spacings.xl,
    },
  });

  return styles;
}
