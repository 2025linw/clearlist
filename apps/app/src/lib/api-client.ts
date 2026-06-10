import { Platform } from 'react-native';

import { authClient } from '@/lib/auth-client';

export type Response = {
  message?: string;
};

export class ApiError extends Error {
  readonly status: number;
  readonly statusText: string;

  constructor(message: string, status: number, statusText: string) {
    super(message);

    this.name = 'ApiError';
    this.status = status;
    this.statusText = statusText;

    Object.setPrototypeOf(this, ApiError.prototype);
  }
}

export async function apiFetch<T>(
  input: string,
  init?: RequestInit,
): Promise<T> {
  const headers = init?.headers ? new Headers(init.headers) : new Headers();
  if (Platform.OS !== 'web') {
    const cookie = authClient.getCookie();

    if (cookie) {
      headers.set('Cookie', cookie);
    }
  }

  const res = await fetch(input, {
    ...init,
    credentials: Platform.OS === 'web' ? 'include' : 'omit',
    headers,
  });
  const contentType = res.headers.get('content-type');
  const isJson = contentType?.includes('application/json');

  if (!isJson) {
    const text = await res.text();

    throw new ApiError(text, res.status, res.statusText);
  }

  if (!res.ok) {
    const json = (await res.json()) as Response;

    console.log(json);
    throw new ApiError(
      json.message ?? 'Request failed',
      res.status,
      res.statusText,
    );
  }

  return (await res.json()) as T;
}
