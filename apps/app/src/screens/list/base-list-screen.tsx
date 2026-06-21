import { useCallback, useRef, useState } from 'react';
import { FlatList, StyleSheet } from 'react-native';

import { TaskDTO } from '@clearlist/types';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';
import * as TaskHook from '@/hooks/use-tasks';
import { categoryQueryMap, toYYYYMMDD } from '@/services/helpers';
import { Category } from '@/services/types';

import Layout from '@/components/layout';
import DateSelectModal from '@/components/modals/date-select-modal';
import Button from '@/components/primitives/button';
import TaskItem from '@/components/task-item';

type Props = {
  headerText: string;
  category: Category;
};

export default function ListScreen(props: Props) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  const searchQuery = categoryQueryMap[props.category];

  const queryTasks = TaskHook.useTasks(searchQuery);
  const createTask = TaskHook.useCreateTask();
  const updateTask = TaskHook.useUpdateTask();
  const completeTask = TaskHook.useCompleteTask();
  const reopenTask = TaskHook.useReopenTask();

  const isFirstRender = useRef(true);
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [draft, setDraft] = useState<TaskDTO | null>(null);
  const [dateModalMode, setDateModalMode] = useState<
    'start' | 'deadline' | null
  >(null);
  const [dateModalDate, setDateModalDate] = useState<string | undefined>(
    undefined,
  );

  function expandTask(id: string) {
    const activeTask = queryTasks.data?.data.tasks.find(
      (task) => task.id === id,
    );
    if (!activeTask) {
      console.error('whoopsie');

      return;
    }

    isFirstRender.current = true;
    setExpandedId(id);
    setDraft({
      title: activeTask.title,
      notes: activeTask.notes,
      start: activeTask.start,
      startPrecision: activeTask.startPrecision,
      deadline: activeTask.deadline,
      tags: activeTask.tags.map((tag) => tag.id),
    });
    // setDateModalDate(activeTask.start ?? undefined);
  }

  function collapseTask() {
    if (!expandedId || !draft) return;

    updateTask.mutate({
      id: expandedId,
      ...draft,
    });

    setDraft(null);
    setExpandedId(null);
  }

  const saveTimeoutRef = useRef<Map<string, ReturnType<typeof setTimeout>>>(
    new Map(),
  );
  const scheduleSave = useCallback(
    (draft: TaskDTO & { id: string }) => {
      const existing = saveTimeoutRef.current.get(draft.id);
      if (existing) {
        clearTimeout(existing);
      }

      const timeout = setTimeout(() => {
        updateTask.mutate(draft);

        saveTimeoutRef.current.delete(draft.id);
      }, 1200);

      saveTimeoutRef.current.set(draft.id, timeout);
    },
    [updateTask],
  );

  const setModalInitDate = useCallback(
    (mode: 'start' | 'deadline') => {
      if (mode === 'start') {
        setDateModalDate(draft?.start || undefined);
      } else if (mode === 'deadline') {
        setDateModalDate(draft?.deadline || undefined);
      }
    },
    [draft?.deadline, draft?.start],
  );

  return (
    <>
      <Layout
        headerText={props.headerText}
        showBackButton
      >
        <FlatList
          data={queryTasks.data?.data.tasks}
          keyExtractor={(item) => item.id}
          renderItem={({ item }) => (
            <TaskItem
              task={item}
              expanded={expandedId === item.id}
              onToggle={() => {
                if (expandedId === item.id) return;
                if (expandedId !== null) {
                  collapseTask();
                  return;
                }

                expandTask(item.id);
              }}
              onTaskUpdate={(patch) => {
                const newDraft = {
                  ...draft,
                  ...patch,
                } as TaskDTO;
                setDraft(newDraft);

                scheduleSave({ id: expandedId!, ...newDraft });
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
        />

        <Button
          text="Add Task"
          iconName="add-circle"
          scheme="primary"
          style={styles.addButton}
          onPress={() => {
            createTask.mutate(
              {
                title: '',
                notes: null,
                start: null,
                startPrecision: 'Date',
                deadline: null,
                tags: [],
              },
              {
                onSuccess: (data) => {
                  setExpandedId(data.data.id);
                },
              },
            );
          }}
        />
      </Layout>

      <DateSelectModal
        visible={expandedId !== null && dateModalMode !== null}
        initialDate={dateModalDate}
        onDateSelect={(selected) => {
          const newDraft = {
            ...draft,
            ...(dateModalMode === 'start'
              ? { start: selected.toISOString() }
              : { deadline: toYYYYMMDD(selected) }),
          } as TaskDTO;
          setDraft(newDraft);

          scheduleSave({ id: expandedId!, ...newDraft });
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
  const styles = StyleSheet.create({
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
      left: theme.spacings.xl,
      right: theme.spacings.xl,
      zIndex: theme.zHeight.floating,

      borderRadius: theme.rounded.full,
      padding: theme.spacings.lg,
      paddingLeft: theme.spacings.lg + 10,
    },
  });

  return styles;
}
