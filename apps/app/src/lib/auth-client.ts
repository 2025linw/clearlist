import { expoClient } from '@better-auth/expo/client';
import { createAuthClient } from 'better-auth/react';

import * as SecureStore from 'expo-secure-store';

import { API_URL } from '../constants';

export const authClient = createAuthClient({
  baseURL: API_URL,
  plugins: [
    expoClient({
      scheme: 'clearlist',
      storagePrefix: 'clearlist',
      cookiePrefix: 'clear_list',
      storage: SecureStore,
    }),
  ],
});
