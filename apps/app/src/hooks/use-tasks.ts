import { useQuery } from '@tanstack/react-query';

import * as taskService from '@/services/tasks';
import { TaskQuery } from '@/services/types';

export function useTasks(query: TaskQuery) {
  return useQuery({
    queryKey: ['tasks', query],
    queryFn: () => taskService.list(query),
  });
}
