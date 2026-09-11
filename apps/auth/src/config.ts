import 'dotenv/config';

export const port = process.env['SRV_PORT']!;

export const apiUrl = process.env['API_URL']!;
export const databaseUrl = process.env['DATABASE_URL']!;

export const webhookSecret = process.env['WEBHOOK_SECRET']!;

export const trustedOrigins = [
  'https://todo.saphydev.com',
  'clearlist://',
  ...(process.env.NODE_ENV === 'development'
    ? ['https://todo.localhost:8443']
    : []),
];
