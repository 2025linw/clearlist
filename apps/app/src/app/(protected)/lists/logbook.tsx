import { useEffect, useState } from 'react';

import { Task } from '@/types';

import { Category, getTasks } from '@/services/api';

import ListScreen from '@/screens/list-screen';

export default function Deadline() {
  const [data, setData] = useState<Task[] | null>(null);

  useEffect(() => {
    getTasks(Category.Logged).then((tasks) => {
      setData(tasks);
    });
  }, []);

  return (
    <ListScreen
      listName={'Logbook'}
      tasks={data}
    />
  );
}
