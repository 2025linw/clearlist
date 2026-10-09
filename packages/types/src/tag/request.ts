import * as z from 'zod';

export const CreateRequestSchema = z.object({
  label: z.string(),
  category: z.string().optional(),

  positionKey: z.string(),
});

export const UpdateRequestSchema = CreateRequestSchema.partial().extend({
  category: z.string().nullable().optional(),
});

export type CreateRequest = z.output<typeof CreateRequestSchema>;
export type UpdateRequest = z.output<typeof UpdateRequestSchema>;
