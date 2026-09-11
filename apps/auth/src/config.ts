import 'dotenv/config';

export const trustedOrigins = [
  'https://todo.saphydev.com',
  'clearlist://',
  ...(process.env.NODE_ENV === 'development'
    ? ['https://todo.localhost:8443']
    : []),
];

export const port = process.env['SRV_PORT'];
