import { betterAuth } from 'better-auth';
import { Pool } from 'pg';

import { expo } from '@better-auth/expo';

import * as config from '../config.ts';
import { createAuthMiddleware } from 'better-auth/api';
import { createHmac, randomUUID } from 'node:crypto';

export const auth = betterAuth({
  trustedOrigins: config.trustedOrigins,

  database: new Pool({
    connectionString: config.databaseUrl,
    max: 10,
  }),

  plugins: [expo()],
  emailAndPassword: {
    enabled: true,
  },

  hooks: {
    after: createAuthMiddleware(async (ctx) => {
      if (ctx.path.startsWith('/sign-up')) {
        const session = ctx.context.newSession;
        if (!session) return;

        const body = JSON.stringify({
          id: session.user.id,
          displayName: session.user.name,
          createdAt: session.user.createdAt,
        });

        const webhookId = randomUUID();
        const timestamp = Math.floor(Date.now() / 1000).toString();

        const payload = `${webhookId}.${timestamp}.${body}`;

        const signature = createHmac('sha256', config.webhookSecret)
          .update(payload)
          .digest('hex');

        fetch(config.apiUrl + '/internal/users/provision', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'X-Webhook-Id': webhookId,
            'X-Webhook-Timestamp': timestamp,
            'X-Webhook-Signature': `v1=${signature}`,
          },
          body,
        }).then(
          (res) => {
            console.log('success: ' + res.status);
          },
          (err) => {
            console.error('fail' + err);
          },
        );
      }
    }),
  },

  advanced: {
    useSecureCookies: true,

    defaultCookieAttributes: {
      sameSite: 'lax',
      secure: true,
      httpOnly: true,
    },

    database: {
      generateId: () => crypto.randomUUID(),
    },
  },
});
