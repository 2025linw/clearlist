import { Platform } from 'react-native';

import { authClient } from '@/lib/auth-client';

// export type Response = {
//   message?: string;
// };

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

export async function apiFetch(
  input: string,
  init?: RequestInit,
): Promise<Response> {
  const headers = init?.headers ? new Headers(init.headers) : new Headers();
  if (Platform.OS !== 'web') {
    const cookie = authClient.getCookie();

    if (cookie) {
      headers.set('Cookie', cookie);
    }
  }

  return fetch(input, {
    ...init,
    credentials: Platform.OS === 'web' ? 'include' : 'omit',
    headers,
  });
}
