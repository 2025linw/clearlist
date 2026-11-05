import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

import {
  type CreateRequest,
  type TaskResponse,
  type UpdateRequest,
} from '@compose/types/task';

import {
  completeTask,
  createTask,
  getTasks,
  reopenTask,
  restoreTask,
  trashTask,
  updateTask,
} from '@services/api/tasks';
import { type TaskQuery } from '@services/api/types';

export function useTasks(query: TaskQuery) {
  return useQuery({
    queryKey: ['tasks', query],
    queryFn: () => getTasks(query),
  });
}

export function useCreateTask() {
  const queryClient = useQueryClient();

  return useMutation<TaskResponse, Error, CreateRequest>({
    mutationFn: (payload) => createTask(payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useUpdateTask() {
  const queryClient = useQueryClient();

  return useMutation<
    TaskResponse,
    Error,
    { id: string; payload: UpdateRequest }
  >({
    mutationFn: (request) => updateTask(request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useCompleteTask() {
  const queryClient = useQueryClient();

  return useMutation<unknown, Error, string>({
    mutationFn: (id) => completeTask(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useReopenTask() {
  const queryClient = useQueryClient();

  return useMutation<unknown, Error, string>({
    mutationFn: (id) => reopenTask(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useTrashTask() {
  const queryClient = useQueryClient();

  return useMutation<unknown, Error, string>({
    mutationFn: (id) => trashTask(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useRestoreTask() {
  const queryClient = useQueryClient();

  return useMutation<unknown, Error, string>({
    mutationFn: (id) => restoreTask(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}
