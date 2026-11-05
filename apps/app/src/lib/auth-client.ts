import { expoClient } from '@better-auth/expo/client';
import { createAuthClient } from 'better-auth/react';

import * as SecureStore from 'expo-secure-store';

const API_URL = process.env.EXPO_PUBLIC_API_URL;

export const authClient = createAuthClient({
  baseURL: API_URL,
  plugins: [
    expoClient({
      scheme: 'compose',
      storagePrefix: 'compose',
      cookiePrefix: 'compose',
      storage: SecureStore,
    }),
  ],
});
