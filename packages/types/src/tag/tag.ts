import * as z from 'zod';

import { CategorySchema } from '../category';
import { DatetimeSchema } from '../common';

export const TagSchema = z.object({
  id: z.uuid(),

  label: z.string(),
  category: CategorySchema.optional(),

  positionKey: z.string(),

  updatedAt: DatetimeSchema,
  createdAt: DatetimeSchema,
  createdBy: z.uuid(),
});

export type Tag = z.infer<typeof TagSchema>;
