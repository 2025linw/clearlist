import * as z from 'zod';

import { TagSchema } from './tag';
import { ResponseSchema as BaseResponseSchema } from '../common';

export const TagResponseSchema = BaseResponseSchema.extend({
  data: TagSchema,
});

export const TagQueryResponseSchema = BaseResponseSchema.extend({
  data: z.object({
    count: z.number(),
    tags: z.array(TagSchema),
  }),
});

export type TagResponse = z.output<typeof TagResponseSchema>;
export type TagQueryResponse = z.output<typeof TagQueryResponseSchema>;
