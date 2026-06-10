import Constants from 'expo-constants';
import { Platform } from 'react-native';

const envAPIURL = process.env.EXPO_PUBLIC_API_URL;
export const API_URL = (() => {
  if (envAPIURL) return envAPIURL;

  if (!__DEV__) {
    throw new Error('Missing EXPO_PUBLIC_API_URL');
  }

  if (Platform.OS === 'web') {
    return 'https://todo.localhost:8081';
  }

  const hostIp = Constants.expoConfig?.hostUri?.split(':')[0];
  if (!hostIp) {
    throw new Error('Unable to determine Expo host IP');
  }

  return `http://${hostIp}:8080`;
})();
