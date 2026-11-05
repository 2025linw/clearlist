import {
  type CreateRequest,
  CreateRequestSchema,
  type TaskQueryResponse,
  TaskQueryResponseSchema,
  type TaskResponse,
  TaskResponseSchema,
  type UpdateRequest,
  UpdateRequestSchema,
} from '@compose/types/task';

import { api } from './client';
import { buildTaskQuery } from './helpers';
import { type TaskQuery } from './types';

export async function getTasks(query: TaskQuery, signal?: AbortSignal) {
  const params = buildTaskQuery(query);

  const res = await api.get<TaskQueryResponse>('/api/tasks', {
    query: params,
    signal,
  });

  return TaskQueryResponseSchema.parse(res);
}

export async function createTask(payload: CreateRequest, signal?: AbortSignal) {
  const body = CreateRequestSchema.encode(payload);

  const res = await api.post<TaskResponse>('/api/tasks', body, { signal });

  return TaskResponseSchema.parse(res);
}

export async function updateTask(
  request: { id: string; payload: UpdateRequest },
  signal?: AbortSignal,
) {
  const body = UpdateRequestSchema.encode(request.payload);

  const res = await api.patch<TaskResponse>(`/api/tasks/${request.id}`, body, {
    signal,
  });

  return TaskResponseSchema.parse(res);
}

export async function trashTask(id: string, signal?: AbortSignal) {
  return await api.delete<Response>(`/api/tasks/${id}`, {
    signal,
  });
}

export async function restoreTask(id: string, signal?: AbortSignal) {
  return await api.post<Response>(`/api/tasks/${id}/restore`, {
    signal,
  });
}

export async function completeTask(id: string, signal?: AbortSignal) {
  return await api.post<Response>(`/api/tasks/${id}/complete`, {
    signal,
  });
}

export async function reopenTask(id: string, signal?: AbortSignal) {
  return await api.post<Response>(`/api/tasks/${id}/reopen`, {
    signal,
  });
}
