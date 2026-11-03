import * as z from 'zod';

import dayjs, { Dayjs } from 'dayjs';

const StartValueSchema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('date'),
    value: z.custom<Dayjs>(dayjs.isDayjs),
  }),
  z.object({
    type: z.literal('datetime'),
    value: z.custom<Dayjs>(dayjs.isDayjs),
  }),
]);

export const StartSchema = z.codec(
  z.union([z.iso.date(), z.iso.datetime({ offset: true })]),
  StartValueSchema,
  {
    decode: (value) => {
      if (value.includes('T')) {
        return {
          type: 'datetime' as const,
          value: dayjs(value),
        };
      }

      return {
        type: 'date' as const,
        value: dayjs(value),
      };
    },
    encode: (start) => {
      if (start.type === 'datetime') {
        return start.value.toISOString();
      }

      return start.value.format('YYYY-MM-DD');
    },
  },
);

export type Start = z.output<typeof StartSchema>;
