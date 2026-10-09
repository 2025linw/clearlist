import * as z from 'zod';

export const CategorySchema = z.object({
  id: z.uuid(),
  name: z.string(),
  positionKey: z.string(),
});

export type Category = z.infer<typeof CategorySchema>;
