import 'dotenv/config';

export const trustedOrigins =
  process.env['NODE_ENV'] === 'development'
    ? ['https://todo.localhost:8081', 'clearlist://']
    : ['https://todo.saphynet.io', 'clearlist://'];

export const port = process.env['SRV_PORT'];
