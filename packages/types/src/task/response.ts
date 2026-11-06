import * as z from 'zod';

import { TaskSchema } from './task';
import { ResponseSchema as BaseResponseSchema } from '../common';

export const TaskResponseSchema = BaseResponseSchema.extend({
  data: TaskSchema,
});

export const TaskQueryResponseSchema = BaseResponseSchema.extend({
  data: z.object({
    count: z.number(),
    tasks: z.array(TaskSchema),
  }),
});

export type TaskResponse = z.output<typeof TaskResponseSchema>;
export type TaskQueryResponse = z.output<typeof TaskQueryResponseSchema>;
