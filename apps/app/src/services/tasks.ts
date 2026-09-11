import { task } from '@clearlist/types';

import { API_URL } from '@/constants';

import { buildTaskQuery } from '@/services/helpers';

import { apiFetch } from '@/lib/api-client';

import { TaskQuery, TaskQueryResponse, TaskResponse } from './types';

export async function create(task: task.CreateRequest): Promise<TaskResponse> {
  const res = await apiFetch(API_URL + '/api/tasks', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(task),
  });

  return res.json();
}

export async function get(id: string): Promise<TaskResponse> {
  const res = await apiFetch(API_URL + `/api/tasks/${id}`);

  return res.json();
}

export async function list(query: TaskQuery = {}): Promise<TaskQueryResponse> {
  const qs = buildTaskQuery(query);

  const res = await apiFetch(
    qs ? API_URL + `/api/tasks?${qs}` : API_URL + '/api/tasks',
  );

  return res.json();
}

export async function update({
  id,
  update: task,
}: {
  id: string;
  update: task.UpdateRequest;
}): Promise<TaskResponse> {
  const res = await apiFetch(API_URL + `/api/tasks/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(task),
  });

  return res.json();
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
