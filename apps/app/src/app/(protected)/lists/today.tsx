import { useEffect, useState } from 'react';

import { Task } from '@clearlist/types';

import { Category, getTasks } from '@/services/api';

import ListScreen from '@/screens/list-screen';

export default function Today() {
  const [data, setData] = useState<Task[] | null>(null);

  useEffect(() => {
    getTasks(Category.Today).then((tasks) => {
      setData(tasks);
    });
  }, []);

  return (
    <ListScreen
      listName={'Today'}
      tasks={data}
      emptyText="Create a new task!"
    />
  );
}
