import * as z from 'zod';
import { DatetimeSchema } from '../common';

export const UserSchema = z.object({
  id: z.uuid(),

  displayName: z.string(),

  preferredTimezone: z.string().optional(),
  completedTaskRetention: z.string().optional(),

  updatedAt: DatetimeSchema,
  createdAt: DatetimeSchema,
});

export type User = z.infer<typeof UserSchema>;
