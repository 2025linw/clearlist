import 'dotenv/config'

export const trustedOrigins = [
  'https://todo.localhost:8443',
  'https://todo.saphynet.io',
  'clearlist://',
];

export const port = process.env['SRV_PORT'];
