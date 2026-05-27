import Constants from 'expo-constants';
import { Platform } from 'react-native';

const IS_DEV = __DEV__;

const hostUri = Constants.expoConfig?.hostUri;
const hostIp = hostUri?.split(':')[0];

export const API_URL = !IS_DEV
  ? 'https://todo.saphynet.io'
  : Platform.OS === 'web'
    ? 'https://todo.localhost:8443'
    : `http://${hostIp}:8443`;
