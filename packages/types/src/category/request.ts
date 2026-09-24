import * as z from 'zod';

export const CreateRequestSchema = z.object({
  name: z.string(),

  positionKey: z.string(),
});
export const UpdateRequestSchema = CreateRequestSchema.partial();

export type CreateRequest = z.output<typeof CreateRequestSchema>;
export type UpdateRequest = z.output<typeof UpdateRequestSchema>;
