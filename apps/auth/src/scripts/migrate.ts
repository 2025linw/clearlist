import 'dotenv/config';

import { betterAuth } from 'better-auth';
import { getMigrations } from 'better-auth/db/migration';
import { PostgresDialect } from 'kysely';
import { Pool } from 'pg';

const migrationAuth = betterAuth({
  database: {
    dialect: new PostgresDialect({
      pool: new Pool({
        connectionString: process.env.MIGRATION_URL,
        max: 10,
      }),
    }),
    type: 'postgres',
    schemaName: 'auth',
  },
});

const { toBeCreated, toBeAdded, runMigrations } = await getMigrations(
  migrationAuth.options,
);

if (toBeCreated.length === 0 && toBeAdded.length === 0) {
  console.log('No migrations needed');

  process.exit(0);
}

await runMigrations();

console.log('Migration ran sucessfully');
process.exit(0);
