import { useEffect, useState } from 'react';

import { Task } from '@/types';

import { getTasks } from '@/services/api';

import ListScreen from '@/screens/list-screen';

export default function Inbox() {
  const [data, setData] = useState<Task[] | null>(null);

  useEffect(() => {
    getTasks().then((tasks) => {
      setData(tasks);
    });
  }, []);

  return (
    <ListScreen
      listName={'Debug'}
      tasks={data}
    />
  );
}
