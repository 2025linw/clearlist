import { useEffect, useState } from 'react';

import { Task } from '@clearlist/types';

import { Category, getTasks } from '@/services/api';

import ListScreen from '@/screens/list-screen';

export default function Inbox() {
  const [data, setData] = useState<Task[] | null>(null);

  useEffect(() => {
    getTasks(Category.Inbox).then((tasks) => {
      setData(tasks);
    });
  }, []);

  return (
    <ListScreen
      listName={'Inbox'}
      tasks={data}
      emptyText="Create a new task!"
    />
  );
}
