import * as z from 'zod';

import { TaskSchema } from './task';
import { ResponseSchema as BaseResponseSchema } from '../common';

export const ResponseSchema = BaseResponseSchema.extend({
  data: TaskSchema,
});

export const QueryResponseSchema = BaseResponseSchema.extend({
  data: z.object({
    count: z.number(),
    tasks: z.array(TaskSchema),
  }),
});

export type Response = z.output<typeof ResponseSchema>;
export type QueryResponse = z.output<typeof QueryResponseSchema>;
