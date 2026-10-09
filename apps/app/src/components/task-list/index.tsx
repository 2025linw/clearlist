import { useCallback, useEffect, useRef, useState } from 'react';
import { type ListRenderItemInfo, StyleSheet, View } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, {
  LinearTransition,
  useSharedValue,
} from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';

import { type task } from '@clearlist/types';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import type dayjs from '@lib/datetime';

import DateSelectModal from '@components/modals/date-select-modal';

import Card, { CARD_TRANSITION_SPEED } from './card';
import MenuButton from './menu-button';

type TaskListProp = {
  data?: task.Task[];
  onAddTask?: (onCreateCallback: (id: string) => void) => void;
  onTaskUpdate?: (id: string, update: Partial<task.UpdateRequest>) => void;
  onTaskComplete?: (id: string) => void;
  onTaskReopen?: (id: string) => void;
  onTaskTrash?: (id: string) => void;
  onTaskRestore?: (id: string) => void;
};

export default function TaskList({
  data,
  onAddTask,
  onTaskUpdate,
  onTaskComplete,
  onTaskReopen,
  onTaskTrash,
  onTaskRestore,
}: TaskListProp) {
  const theme = useTheme();

  const taskAdded = useRef<string | null>(null);
  const [expandedTask, setExpandedTask] = useState<task.Task | null>(null);
  const [dateModal, setDateModal] = useState<{
    taskId: string;
    state: 'start' | 'deadline';
    initialDate?: dayjs.Dayjs;
  } | null>(null);
  const touchedExpandedCard = useSharedValue(false);

  const styles = buildStyles(theme);
  const activeExpandedTask = data?.some(({ id }) => id === expandedTask?.id)
    ? expandedTask
    : null;

  const handleTaskPress = useCallback(
    (id: string) => {
      if (activeExpandedTask === null) {
        setExpandedTask(data?.find((task) => task.id === id) || null);
      } else if (activeExpandedTask.id !== id) {
        setExpandedTask(null);
      }
    },
    [data, activeExpandedTask],
  );
  const addTask = useCallback(() => {
    onAddTask?.((id) => {
      taskAdded.current = id;
    });
  }, [taskAdded, onAddTask]);

  useEffect(() => {
    const newTask = data?.find((task) => taskAdded.current === task.id);
    if (newTask) {
      setExpandedTask(newTask);
    }

    taskAdded.current = null;
  }, [taskAdded, data]);

  const tapGesture = Gesture.Tap()
    .maxDistance(5)
    .cancelsTouchesInView(false)
    .onEnd((_e, success) => {
      if (success && !touchedExpandedCard.get()) {
        scheduleOnRN(setExpandedTask, null);
      }
    });

  const renderItem = useCallback(
    ({ item }: ListRenderItemInfo<task.Task>) => {
      const expanded = item.id === activeExpandedTask?.id;

      return (
        <View
          onTouchStart={() => {
            touchedExpandedCard.set(expanded);
          }}
          onStartShouldSetResponderCapture={() => {
            if (expanded) {
              touchedExpandedCard.set(true);
            }

            return false;
          }}
        >
          <Card
            task={item}
            expanded={expanded}

            onPress={() => handleTaskPress(item.id)}

            onTaskUpdate={(patch) => onTaskUpdate?.(item.id, patch)}
            onTaskComplete={() => onTaskComplete?.(item.id)}
            onTaskReopen={() => onTaskReopen?.(item.id)}

            onPressStartDate={(date) => {
              setDateModal({
                taskId: item.id,
                state: 'start',
                initialDate: date,
              });
            }}
            onPressDeadline={(date) => {
              setDateModal({
                taskId: item.id,
                state: 'deadline',
                initialDate: date,
              });
            }}
          />
        </View>
      );
    },
    [
      touchedExpandedCard,
      activeExpandedTask,
      handleTaskPress,
      onTaskComplete,
      onTaskReopen,
      onTaskUpdate,
    ],
  );

  return (
    <View
      style={styles.container}
      onStartShouldSetResponderCapture={() => {
        touchedExpandedCard.set(false);

        return false;
      }}
    >
      <GestureDetector gesture={tapGesture}>
        <Animated.FlatList
          data={data}
          keyExtractor={(item) => item.id}
          renderItem={renderItem}
          showsVerticalScrollIndicator={false}
          itemLayoutAnimation={LinearTransition.duration(CARD_TRANSITION_SPEED)}
        />
      </GestureDetector>

      <MenuButton
        state={activeExpandedTask ? 'card' : 'list'}
        style={styles.menuButton}
        deleted={!!activeExpandedTask?.deletedAt}
        onAddTask={addTask}
        onTrashTask={() => {
          if (activeExpandedTask) onTaskTrash?.(activeExpandedTask.id);
        }}
        onRestoreTask={() => {
          if (activeExpandedTask) onTaskRestore?.(activeExpandedTask.id);
        }}
      />

      {dateModal && (
        <DateSelectModal
          mode={dateModal.state}
          taskId={dateModal.taskId}
          initialDate={dateModal.initialDate}
          onDateSelect={(selectedDate) => {
            const state = dateModal.state;
            if (state === 'start') {
              onTaskUpdate?.(dateModal.taskId, {
                start: selectedDate
                  ? {
                      type: 'datetime',
                      value: selectedDate,
                    }
                  : null,
              });
            } else {
              onTaskUpdate?.(dateModal.taskId, {
                deadline: selectedDate,
              });
            }

            setDateModal(null);
          }}
          dismiss={() => {
            setDateModal(null);
          }}
        />
      )}
    </View>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      flex: 1,
    },
    menuButton: {
      position: 'absolute',
      bottom: theme.spacings.x8,
      right: theme.spacings.x8,

      zIndex: theme.zHeight.modal,
    },
  });
}
