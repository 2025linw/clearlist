import { ReactElement, useCallback, useRef, useState } from 'react';
import { StyleSheet } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, { LinearTransition } from 'react-native-reanimated';

import { task } from '@clearlist/types';

import { useNotificationContext } from '@/context/error';
import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';
import * as TaskHook from '@/hooks/use-tasks';
import { categoryQueryMap, toYYYYMMDD } from '@/services/helpers';
import { Category } from '@/services/types';

import Icon, { IconProps } from '@/components/icon';
import Layout from '@/components/layout';
import DateSelectModal from '@/components/modals/date-select-modal';
import Button from '@/components/primitives/button';
import TaskCard from '@/components/task-card';

const nullDraft: task.UpdateRequest = {
  title: null,
  notes: null,
  start: null,
  deadline: null,
  tags: null,
  positionKey: null,
};

type ListScreenProps = {
  listName: string;
  listIcon?: ReactElement<IconProps>;
  category: Category;
};

export default function ListScreen(props: ListScreenProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);
  // const { showError } = useNotificationContext();

  const searchQuery = categoryQueryMap[props.category];

  const queryTasks = TaskHook.useTasks(searchQuery);
  const createTask = TaskHook.useCreateTask();
  const updateTask = TaskHook.useUpdateTask();
  const completeTask = TaskHook.useCompleteTask();
  const reopenTask = TaskHook.useReopenTask();

  const [expandedTask, setExpandedTask] = useState<{
    id: string;
    draft: task.UpdateRequest;
  } | null>(null);
  const [dateModalMode, setDateModalMode] = useState<
    'start' | 'deadline' | null
  >(null);
  const [dateModalDate, setDateModalDate] = useState<string | undefined>(
    undefined,
  );

  function hasChanges(update: task.UpdateRequest) {
    return (
      update.title ||
      update.notes ||
      update.start ||
      update.deadline ||
      update.tags ||
      update.positionKey
    );
  }

  function onToggle(id: string) {
    if (!expandedTask) return;
    if (expandedTask.id === id) return;

    const expandedId = expandedTask.id;
    const draft = expandedTask.draft;
    if (expandedId && draft && hasChanges(draft)) {
      updateTask.mutate({
        id: expandedId,
        update: draft,
      });
    }

    setExpandedTask({
      id,
      draft: nullDraft,
    });
  }

  function dismissExpanded() {
    if (!expandedTask) return;

    setExpandedTask(null);
  }
  const dismissTasks = Gesture.Tap().onStart(dismissExpanded);

  const saveTimeoutRef = useRef<Map<string, ReturnType<typeof setTimeout>>>(
    new Map(),
  );
  const scheduleSave = useCallback(
    ({ id, draft }: { id: string; draft: task.UpdateRequest }) => {
      const existing = saveTimeoutRef.current.get(id);
      if (existing) {
        clearTimeout(existing);
      }

      const timeout = setTimeout(() => {
        updateTask.mutate({ id, update: draft });

        saveTimeoutRef.current.delete(id);
      }, 1200);

      saveTimeoutRef.current.set(id, timeout);
    },
    [updateTask],
  );

  const setModalInitDate = useCallback(
    (mode: 'start' | 'deadline') => {
      if (mode === 'start') {
        setDateModalDate(expandedTask?.draft?.start || undefined);
      } else if (mode === 'deadline') {
        setDateModalDate(expandedTask?.draft?.deadline || undefined);
      }
    },
    [expandedTask?.draft?.start, expandedTask?.draft?.deadline],
  );

  return (
    <>
      <GestureDetector gesture={dismissTasks}>
        <Layout
          headerText={props.listName}
          headerIcon={props.listIcon}
          showBackButton
        >
          <Animated.FlatList
            data={queryTasks.data?.data.tasks}
            keyExtractor={(item) => item.id}
            renderItem={({ item }) => (
              <TaskCard
                task={item}
                expanded={expandedTask?.id === item.id}
                onToggle={() => onToggle(item.id)}
                onTaskUpdate={(patch) => {
                  if (!expandedTask) return;

                  const newDraft = {
                    ...expandedTask.draft,
                    ...patch,
                  } as task.UpdateRequest;
                  scheduleSave(expandedTask);
                  setExpandedTask({ ...expandedTask, draft: newDraft });
                }}
                onTaskComplete={() => completeTask.mutate(item.id)}
                onTaskReopen={() => reopenTask.mutate(item.id)}
                onPressStartDate={() => {
                  setDateModalMode('start');
                  setModalInitDate('start');
                }}
                onPressDeadline={() => {
                  setDateModalMode('deadline');
                  setModalInitDate('deadline');
                }}
              />
            )}
            style={styles.container}
            showsVerticalScrollIndicator={false}
            itemLayoutAnimation={LinearTransition.duration(200)}
          />

          <Button
            scheme="primary"
            style={styles.addButton}
            icon={
              <Icon
                name="add-circle"
                color="white"
                size={30}
              />
            }
            onPress={() => {
              createTask.mutate(
                {
                  title: '',
                  notes: null,
                  start: null,
                  deadline: null,
                  tags: [],
                  positionKey: 'a',
                },
                {
                  onSuccess: (data) => {
                    setExpandedTask({ id: data.data.id, draft: nullDraft });
                  },
                },
              );
            }}
          >
            Add Task
          </Button>
        </Layout>
      </GestureDetector>

      <DateSelectModal
        visible={expandedTask !== null && dateModalMode !== null}
        initialDate={dateModalDate}
        onDateSelect={(selected) => {
          if (!expandedTask) return;

          const newDraft = {
            ...expandedTask.draft,
            ...(dateModalMode === 'start'
              ? { start: selected.toISOString() }
              : { deadline: toYYYYMMDD(selected) }),
          } as task.UpdateRequest;
          scheduleSave(expandedTask);
          setExpandedTask({ ...expandedTask, draft: newDraft });
          setDateModalMode(null);
        }}
        dismiss={() => {
          setDateModalMode(null);
        }}
      />
    </>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      ...StyleSheet.absoluteFill,
    },
    emptyComponent: {
      flex: 1,

      justifyContent: 'center',
      alignItems: 'center',
    },
    addButton: {
      position: 'absolute',
      bottom: 0,
      left: theme.spacings.x4,
      right: theme.spacings.x4,
      zIndex: theme.zHeight.floating,

      borderRadius: theme.rounded.full,
      padding: theme.spacings.x2,
    },
  });
}
