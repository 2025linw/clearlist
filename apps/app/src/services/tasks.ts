import { task } from '@clearlist/types';

import { apiFetch } from '@lib/api';
import { buildTaskQuery } from '@services/helpers';

import { API_URL } from '@/constants';

import { TaskQuery } from './types';

export async function create(
  create: task.CreateRequest,
): Promise<task.Response> {
  const body = task.CreateRequestSchema.encode(create);

  const res = await apiFetch(API_URL + '/api/tasks', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });

  const json: unknown = await res.json();
  return task.ResponseSchema.parse(json);
}

export async function get(id: string) {
  const res = await apiFetch(API_URL + `/api/tasks/${id}`);

  const json: unknown = await res.json();
  return task.ResponseSchema.parse(json);
}

export async function list(query: TaskQuery = {}): Promise<task.QueryResponse> {
  const qs = buildTaskQuery(query);

  const res = await apiFetch(
    qs ? API_URL + `/api/tasks?${qs}` : API_URL + '/api/tasks',
  );

  const json: unknown = await res.json();
  return task.QueryResponseSchema.parse(json);
}

export async function update({
  id,
  update,
}: {
  id: string;
  update: Partial<task.UpdateRequest>;
}): Promise<task.Response> {
  const body = task.UpdateRequestSchema.encode(update);

  const res = await apiFetch(API_URL + `/api/tasks/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });

  const json: unknown = await res.json();
  return task.ResponseSchema.parse(json);
}

export async function trash(id: string) {
  return await apiFetch(API_URL + `/api/tasks/${id}`, {
    method: 'DELETE',
  });
}

export async function restore(id: string) {
  return await apiFetch(API_URL + `/api/tasks/${id}/restore`, {
    method: 'POST',
  });
}

export async function complete(id: string): Promise<Response> {
  return apiFetch(API_URL + `/api/tasks/${id}/complete`, {
    method: 'POST',
  });
}

export async function reopen(id: string): Promise<Response> {
  return apiFetch(API_URL + `/api/tasks/${id}/reopen`, {
    method: 'POST',
  });
}
