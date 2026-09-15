import { Platform } from 'react-native';

import { Tag, Task, TaskDTO } from '@clearlist/types';

import { API_URL } from '@/constants';

import { authClient } from '@/lib/auth-client';

import { BaseResponse, TagResponse, TaskResponse } from './types';

export function apiFetch(
  input: string,
  { method, headers, ...init }: RequestInit = {},
): Promise<Response> {
  if (Platform.OS === 'web') {
    return fetch(input, {
      method,
      credentials: 'include',
      headers,
      ...init,
    });
  }

  const cookie = authClient.getCookie();
  return fetch(input, {
    method,
    credentials: 'omit',
    headers: {
      cookie,
      ...headers,
    },
    ...init,
  });
}

function toYYYYMMDD(date: Date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');

  return `${year}-${month}-${day}`;
}

export enum Category {
  Inbox,
  Today,
  Upcoming,
  Deadline,
  Logged,
  Trash,
}

const categoryQueryMap: Record<Category, string> = {
  [Category.Inbox]: '/api/tasks?start=false&deadline=false',
  [Category.Today]: `/api/tasks?start=${toYYYYMMDD(new Date())}`,
  [Category.Upcoming]: `/api/tasks?start[>]=${toYYYYMMDD(new Date())}`,
  [Category.Deadline]: '/api/tasks?deadline=true',
  [Category.Logged]: '/api/tasks?completed=true',
  [Category.Trash]: '/api/tasks?deleted=true',
};

export async function getTasks(category?: Category): Promise<Task[]> {
  let url: string = '/api/tasks';
  if (category !== undefined) {
    url = categoryQueryMap[category];
  }
  url = API_URL + url;

  const res = await apiFetch(url);
  if (res.status !== 200) {
    const body: BaseResponse = await res.json();

    console.error(body.message);

    return [];
  }

  const body: TaskResponse = await res.json();

  return body.data.tasks;
}

export async function addTasks(task: TaskDTO): Promise<void> {
  let url = API_URL + '/api/tasks';

  const res = await apiFetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(task),
  });
  if (res.status !== 201) {
    const body: BaseResponse = await res.json();

    console.error(body.message);

    return;
  }

  const body: TaskResponse = await res.json();
  console.log(body);

  return;
}

export async function getTags(): Promise<Tag[]> {
  const url = API_URL + '/api/tags';

  const res = await apiFetch(url);
  if (res.status !== 200) {
    const body: BaseResponse = await res.json();

    console.error(body.message);

    return [];
  }

  const body: TagResponse = await res.json();

  return body.data.tags;
}
