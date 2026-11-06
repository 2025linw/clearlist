import * as z from 'zod';

import { CategorySchema } from './category';
import { ResponseSchema as BaseResponseSchema } from '../common';

export const CategoryResponseSchema = BaseResponseSchema.extend({
  data: CategorySchema,
});

export const CategoryQueryResponseSchema = BaseResponseSchema.extend({
  data: z.object({
    count: z.number(),
    categories: z.array(CategorySchema),
  }),
});

export type CategoryResponse = z.output<typeof CategoryResponseSchema>;
export type CategoryQueryResponse = z.output<typeof CategoryQueryResponseSchema>;
