import {
  type CreateRequest,
  CreateRequestSchema,
  type QueryResponse,
  type Response,
  ResponseSchema,
  type UpdateRequest,
  UpdateRequestSchema,
} from '@compose/types/task';

import { api } from './client';
import { buildTaskQuery } from './helpers';
import { type TaskQuery } from './types';

export async function getTasks(query: TaskQuery, signal?: AbortSignal) {
  const params = buildTaskQuery(query);

  return api.get<QueryResponse>('/api/tasks', {
    query: params,
    signal,
  });
}

export async function createTask(payload: CreateRequest, signal?: AbortSignal) {
  const body = CreateRequestSchema.encode(payload);

  const res = await api.post<Response>('/api/tasks', body, { signal });

  return ResponseSchema.parse(res);
}

export async function updateTask(
  request: { id: string; payload: UpdateRequest },
  signal?: AbortSignal,
) {
  const body = UpdateRequestSchema.encode(request.payload);

  const res = await api.patch<Response>(`/api/tasks/${request.id}`, body, {
    signal,
  });

  return ResponseSchema.parse(res);
}

export async function trashTask(id: string) {}

export async function restoreTask(id: string) {}

export async function completeTask(id: string) {}

export async function reopenTask(id: string) {}
