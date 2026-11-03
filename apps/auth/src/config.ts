import 'dotenv/config';

export const port = process.env['SRV_PORT']!;

export const apiUrl = process.env['API_URL'];
export const databaseUrl = process.env['DATABASE_URL']!;

export const webhookSecret = process.env['WEBHOOK_SECRET']!;

export const allowedOrigins = process.env['ALLOWED_ORIGINS']
  ? process.env['ALLOWED_ORIGINS'].split(',')
  : [];
