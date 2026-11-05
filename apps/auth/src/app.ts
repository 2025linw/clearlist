import express, { type Request, type Response } from 'express';
import cors, { type CorsOptions } from 'cors';
import { toNodeHandler } from 'better-auth/node';

import * as config from './config.ts';
import { auth } from './lib/auth.ts';

class HttpError extends Error {
  readonly status: number;

  constructor(message: string, status: number) {
    super(message);
    this.status = status;

    Object.setPrototypeOf(this, HttpError.prototype);
  }
}

const app = express();

// CORS
const corsOptions: CorsOptions = {
  origin: (origin, callback) => {
    if (!origin || config.allowedOrigins.includes(origin)) {
      callback(null, true);
    } else {
      callback(new HttpError('Not allowed by CORS', 403));
    }
  },
  allowedHeaders: ['Content-Type', 'Authorization'],
  methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', 'OPTIONS'],
  credentials: true,
};
app.use(cors(corsOptions));

app.all('/api/auth/*splat', toNodeHandler(auth));

// Error handler
app.use((err: Error, req: Request, res: Response) => {
  let statusCode = 500;

  if (err instanceof HttpError) {
    statusCode = err.status;
  } else {
    console.error('Unexpected Error:', err.stack);
  }

  res.status(statusCode).json({
    message: err.message || 'An unexpected error occured.',
  });
});

app.listen(config.port, (error?: Error) => {
  if (error) {
    console.error(error);

    process.exit(1);
  }

  console.log(`Server running on port ${config.port}`);
});
