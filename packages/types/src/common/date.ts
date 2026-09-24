import dayjs, { Dayjs } from 'dayjs';
import * as z from 'zod';

export const DateSchema = z.codec(
  z.iso.date(),
  z.custom<Dayjs>(dayjs.isDayjs),
  {
    decode: (value) => dayjs(value),
    encode: (value) => value.format('YYYY-MM-DD'),
  },
);
