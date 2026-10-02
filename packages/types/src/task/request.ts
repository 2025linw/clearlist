import * as z from 'zod';

import { DateSchema, StartSchema } from '../common';

export const CreateRequestSchema = z.object({
  title: z.string(),
  notes: z.string().optional(),
  start: StartSchema.optional(),
  deadline: DateSchema.optional(),
  tags: z.array(z.uuid()),

  positionKey: z.string(),
});

export const UpdateRequestSchema = CreateRequestSchema.partial().extend({
  notes: z.string().nullable().optional(),
  start: StartSchema.nullable().optional(),
  deadline: DateSchema.nullable().optional(),
});

export type CreateRequest = z.output<typeof CreateRequestSchema>;
export type UpdateRequest = z.output<typeof UpdateRequestSchema>;
