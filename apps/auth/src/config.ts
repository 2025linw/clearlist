import 'dotenv/config';

export const trustedOrigins = [
  'https://todo.localhost:8443',
  'https://todo.saphydev.com',
  'clearlist://',
];

export const port = process.env['SRV_PORT'];
