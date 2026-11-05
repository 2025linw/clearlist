import { Platform } from 'react-native';

import { authClient } from '@lib/auth-client';

import { type QueryParams } from './types';

const API_URL = process.env.EXPO_PUBLIC_API_URL;

type RequestOptions = {
  query?: QueryParams;
  signal?: AbortSignal;
};

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly data: unknown,
  ) {
    super(`API request failed (${status})`);
    this.name = 'ApiError';
  }
}

type MethodType = 'GET' | 'POST' | 'PATCH' | 'PUT' | 'DELETE';

async function request<T>(
  method: MethodType,
  path: string,
  body: unknown,
  options: RequestOptions = {},
): Promise<T> {
  if (!API_URL) {
    throw new Error('EXPO_PUBLIC_API_URL is not configured');
  }

  const url = new URL(path, API_URL);

  for (const [key, value] of Object.entries(options.query ?? {})) {
    if (value !== undefined && value !== null) {
      url.searchParams.set(key, String(value));
    }
  }

  const headers = new Headers({
    Accept: 'application/json',
  });

  if (body !== undefined) {
    headers.set('Content-Type', 'application/json');
  }

  if (Platform.OS !== 'web') {
    const cookie = await authClient.getCookie();

    if (cookie) {
      headers.set('Cookie', cookie);
    }
  }

  const res = await fetch(url, {
    method,
    headers,
    body: body === undefined ? undefined : JSON.stringify(body),
    credentials: Platform.OS === 'web' ? 'include' : 'omit',
    signal: options.signal,
  });

  const text = await res.text();
  let data: unknown = undefined;

  if (text) {
    try {
      data = JSON.parse(text);
    } catch {
      data = text;
    }
  }

  if (!res.ok) {
    throw new ApiError(res.status, data);
  }

  return data as T;
}

export const api = {
  get<T>(path: string, options?: RequestOptions) {
    return request<T>('GET', path, undefined, options);
  },
  post<T>(path: string, body?: unknown, options?: RequestOptions) {
    return request<T>('POST', path, body, options);
  },
  patch<T>(path: string, body?: unknown, options?: RequestOptions) {
    return request<T>('PATCH', path, body, options);
  },
  put<T>(path: string, body?: unknown, options?: RequestOptions) {
    return request<T>('PUT', path, body, options);
  },
  delete<T>(path: string, options?: RequestOptions) {
    return request<T>('DELETE', path, undefined, options);
  },
};
