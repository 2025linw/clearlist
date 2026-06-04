import { betterAuth } from 'better-auth';
import { Pool } from 'pg';

import { expo } from '@better-auth/expo';

import * as config from '../config.ts';

export const auth = betterAuth({
  trustedOrigins: config.trustedOrigins,

  database: new Pool({
    connectionString: process.env.DATABASE_URL,
    max: 10,
  }),

  plugins: [expo()],
  emailAndPassword: {
    enabled: true,
  },

  advanced: {
    useSecureCookies: true,

    defaultCookieAttributes: {
      sameSite: 'lax',
      secure: true,
      httpOnly: true,
    },

    database: {
      generateId: 'uuid',
    },
  },
});
