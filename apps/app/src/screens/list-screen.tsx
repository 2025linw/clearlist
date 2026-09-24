import { ReactElement } from 'react';

import { task } from '@clearlist/types';

import * as TaskHook from '@hooks/use-tasks';
import dayjs from '@lib/datetime';
import { getCategoryQueryMap } from '@services/helpers';
import { Category } from '@services/types';

import Layout from '@components/layout';
import { IconProps } from '@components/primitives/icon';
import TaskList from '@components/task-list';

const requiredTask: task.CreateRequest = {
  title: '',
  tags: [],
  positionKey: 'm',
};

type CreatableCategory = Extract<
  Category,
  'inbox' | 'today' | 'upcoming' | 'deadline'
>;

type ListScreenProps = {
  listName: string;
  listIcon?: ReactElement<IconProps>;
  category: Category;
};

export default function ListScreen(props: ListScreenProps) {
  // const { showError } = useNotificationContext();

  const searchQuery = getCategoryQueryMap()[props.category];

  const queryTasks = TaskHook.useTasks(searchQuery);

  const createTask = TaskHook.useCreateTask();
  const updateTask = TaskHook.useUpdateTask();
  const completeTask = TaskHook.useCompleteTask();
  const reopenTask = TaskHook.useReopenTask();
  const trashTask = TaskHook.useTrashTask();
  const restoreTask = TaskHook.useRestoreTask();

  const defaultTask: Record<CreatableCategory, () => task.CreateRequest> = {
    inbox: () => requiredTask,
    today: () => ({
      ...requiredTask,
      start: {
        type: 'datetime',
        value: dayjs().startOf('day'),
      },
    }),
    upcoming: () => ({
      ...requiredTask,
      start: {
        type: 'datetime',
        value: dayjs().add(1, 'day').startOf('day'),
      },
    }),
    deadline: () => ({
      ...requiredTask,
      deadline: dayjs().startOf('day'),
    }),
  };

  return (
    <Layout
      headerText={props.listName}
      headerIcon={props.listIcon}
      showBackButton
      safeAreaEdges={['top']}
    >
      <TaskList
        data={queryTasks.data?.data.tasks}

        onAddTask={(onCreateCallback) => {
          const category = props.category;
          if (category !== 'logged' && category !== 'trash')
            createTask.mutate(defaultTask[category](), {
              onSuccess: ({ data }) => {
                onCreateCallback(data.id);
              },
            });
        }}
        onTaskUpdate={(id, update) => updateTask.mutate({ id, update })}
        onTaskComplete={(id) => completeTask.mutate(id)}
        onTaskReopen={(id) => reopenTask.mutate(id)}
        onTaskTrash={(id) => trashTask.mutate(id)}
        onTaskRestore={(id) => restoreTask.mutate(id)}
      />
    </Layout>
  );
}
