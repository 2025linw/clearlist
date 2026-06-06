import { useEffect, useState } from 'react';

import { Task } from '@clearlist/types';

import { Category, getTasks } from '@/services/api';

import ListScreen from '@/screens/list-screen';

export default function Upcoming() {
  const [data, setData] = useState<Task[] | null>(null);

  useEffect(() => {
    getTasks(Category.Upcoming).then((tasks) => {
      setData(tasks);
    });
  }, []);

  return (
    <ListScreen
      listName={'Upcoming'}
      tasks={data}
      emptyText="Create a new task!"
    />
  );
}
