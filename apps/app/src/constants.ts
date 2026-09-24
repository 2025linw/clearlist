const IS_DEV = __DEV__;

export const API_URL = !IS_DEV
  ? 'https://todo.saphydev.com'
  : 'https://will-laptop-kubuntu.tailae0279.ts.net:8080';
