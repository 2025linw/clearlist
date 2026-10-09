import { useEffect, useRef, useState } from 'react';
import { Pressable, StyleSheet, View } from 'react-native';
import Animated, { FadeIn, FadeOut } from 'react-native-reanimated';

import { type task } from '@clearlist/types';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useDebouncedCallback } from '@hooks/use-debounced-callback';
import dayjs from '@lib/datetime';

import EditableTypography from '@components/editable-typography';
import Checkbox from '@components/primitives/checkbox';

import DeadlineBadge from './deadline-badge';
import DeadlineButtonLabel from './deadline-button-label';
import StartBadge from './start-badge';
import StartButtonLabel from './start-button-label';

export const CARD_TRANSITION_SPEED = 250;

type Draft = Pick<task.UpdateRequest, 'title' | 'notes'>;

type TaskCardProps = {
  task: task.Task;
  expanded?: boolean;
  onPress?: () => void;
  disabled?: boolean;

  onTaskComplete?: () => void;
  onTaskReopen?: () => void;
  onTaskUpdate?: (patch: Partial<task.UpdateRequest>) => void;

  onPressStartDate?: (initialDate?: dayjs.Dayjs) => void;
  onPressDeadline?: (initialDate?: dayjs.Dayjs) => void;
  onPressTags?: () => void;
};

export default function TaskCard({
  task,
  expanded = false,
  disabled = false,
  onPress,
  ...props
}: TaskCardProps) {
  const theme = useTheme();

  const wasExpanded = useRef(expanded);
  const draftRef = useRef<Draft>({
    title: task.title,
    notes: task.notes,
  });
  const [draft, setDraft] = useState<
    Pick<task.UpdateRequest, 'title' | 'notes'>
  >({
    title: task.title,
    notes: task.notes,
  });

  const styles = buildStyles(theme);

  const { run: scheduleUpdate, flush: flushUpdate } = useDebouncedCallback(
    (patch: Draft) => {
      props.onTaskUpdate?.(patch);
    },
    800,
  );

  const updateDraft = (patch: Partial<Draft>) => {
    const nextDraft = {
      ...draftRef.current,
      ...patch,
    };

    draftRef.current = nextDraft;
    setDraft(nextDraft);

    scheduleUpdate(nextDraft);
  };

  useEffect(() => {
    const justCollapsed = wasExpanded.current && !expanded;

    if (justCollapsed) {
      flushUpdate();
    }

    wasExpanded.current = expanded;
  }, [expanded, flushUpdate]);

  return (
    <Pressable
      onPress={onPress}
      disabled={disabled}
    >
      <Animated.View
        style={[
          styles.container,
          styles.layout,
          expanded ? styles.expandedContainer : null,
        ]}
        entering={FadeIn}
        exiting={FadeOut}
      >
        <View style={styles.mainRow}>
          <Checkbox
            checked={!!task.completedAt}
            onChange={(checked) => {
              const handler = checked
                ? props.onTaskComplete
                : props.onTaskReopen;

              handler?.();
            }}
          />

          {task.start && !expanded && (
            <StartBadge date={dayjs(task.start.value)} />
          )}

          <View style={styles.titleContainer}>
            <EditableTypography
              value={draft.title}
              onChangeText={(title) => {
                updateDraft({ title });
              }}
              onSave={flushUpdate}
              placeholder="New Task"
              disabled={disabled || !expanded}
              style={styles.titleTypography}
            />
          </View>

          {task.deadline && !expanded && (
            <DeadlineBadge date={dayjs(task.deadline)} />
          )}
        </View>

        {expanded && (
          <Animated.View
            style={styles.layout}
            entering={FadeIn.delay(100).duration(150)}
            exiting={FadeOut.duration(100)}
          >
            <View style={styles.fieldPanel}>
              <View style={styles.inputPanel}>
                <EditableTypography
                  value={draft.notes || undefined}
                  onChangeText={(notes) => {
                    updateDraft({ notes });
                  }}
                  onSave={flushUpdate}
                  placeholder="Notes"
                  disabled={disabled}
                  multiline
                />
              </View>
            </View>

            <View style={styles.buttonSet}>
              {!(task.completedAt && task.deletedAt) && (
                <StartButtonLabel
                  date={task.start?.value || undefined}
                  onPress={() => props.onPressStartDate?.(task.start?.value)}
                />
              )}

              <DeadlineButtonLabel
                date={task.deadline || undefined}
                onPress={() => props.onPressDeadline?.(task.deadline)}
              />

              {/* <Button
                scheme="tertiary"
                icon={
                  <Icon
                  name="pricetag-outline"
                  size={22}
                  />
                  }
                  /> */}
            </View>
          </Animated.View>
        )}
      </Animated.View>
    </Pressable>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      borderRadius: theme.rounded.lg,
      padding: theme.spacings.x3,
    },
    expandedContainer: {
      backgroundColor: theme.palette.surface,
    },
    layout: {
      gap: theme.spacings.x5,
    },
    mainRow: {
      flex: 1,

      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-start',
      alignItems: 'center',
      gap: theme.spacings.x2,
    },
    titleContainer: {
      flex: 1,
    },
    titleTypography: {
      backgroundColor: 'transparent',
      fontSize: 20,
    },
    fieldPanel: {
      display: 'flex',
      flexDirection: 'row',
      gap: theme.spacings.x4,
    },
    inputPanel: {
      flex: 1,

      marginLeft: 18,

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
