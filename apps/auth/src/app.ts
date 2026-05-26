import express from 'express';
import cors from 'cors';

import { toNodeHandler } from 'better-auth/node';

import * as config from './config.ts';
import { auth } from './lib/auth.ts';

const app = express();

app.use(
  cors({
    origin: (origin, callback) => {
      if (!origin || config.trustedOrigins.includes(origin)) {
        callback(null, true);
      } else {
        callback(new Error('Not allowed by CORS'));
      }
    },
    allowedHeaders: ['Content-Type', 'Authorization'],
    methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'],
    credentials: true,
  }),
);

app.all('/api/auth/*splat', toNodeHandler(auth));

app.listen(config.port, (error?: Error) => {
  if (error) {
    console.error(error);

    process.exit(1);
  }

  console.log(`Server running on port ${config.port}`);
});
