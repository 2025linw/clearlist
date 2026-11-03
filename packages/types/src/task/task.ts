import * as z from 'zod';

import { DateSchema, DatetimeSchema, StartSchema } from '../common';
import { TagSchema } from '../tag/tag';

export const TaskSchema = z.object({
  id: z.uuid(),

  title: z.string(),
  notes: z.string().optional(),
  start: StartSchema.optional(),
  deadline: DateSchema.optional(),
  tags: z.array(TagSchema),

  positionKey: z.string(),
  completedAt: DatetimeSchema.optional(),
  deletedAt: DatetimeSchema.optional(),

  updatedAt: DatetimeSchema,
  createdAt: DatetimeSchema,
  createdBy: z.uuid(),
});

export type Task = z.output<typeof TaskSchema>;
