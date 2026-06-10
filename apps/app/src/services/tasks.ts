import { Task } from '@clearlist/types';

import { API_URL } from '@/constants';

import { buildTaskQuery } from '@/services/helpers';

import { apiFetch } from '@/lib/api-client';

import { TaskQuery } from './types';

export async function list(query: TaskQuery = {}) {
  const qs = buildTaskQuery(query);

  return apiFetch<Task[]>(
    qs ? API_URL + `/api/tasks?${qs}` : API_URL + '/api/tasks',
  );
}
