import * as z from 'zod';

export const ResponseSchema = z.object({
  message: z.string().optional(),
});
