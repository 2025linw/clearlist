import * as z from 'zod';

export const UpdateRequestSchema = z.object({
  displayName: z.string().nullable(),

  preferredTimezone: z.string().nullable().optional(),
  completedTaskRetention: z.string().nullable().optional(),
});

export type UpdateRequest = z.output<typeof UpdateRequestSchema>;
