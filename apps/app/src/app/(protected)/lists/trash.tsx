import { useEffect, useState } from 'react';

import { Task } from '@/types';

import { Category, getTasks } from '@/services/api';

import ListScreen from '@/screens/list-screen';

export default function Inbox() {
  const [data, setData] = useState<Task[] | null>(null);

  useEffect(() => {
    getTasks(Category.Trash).then((tasks) => {
      setData(tasks);
    });
  }, []);

  return (
    <ListScreen
      listName={'Trash'}
      tasks={data}
    />
  );
}
