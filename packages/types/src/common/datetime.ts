// import dayjs from 'dayjs';
// import * as z from 'zod';

// export const DatetimeSchema = z.iso.datetime().transform(dayjs);
import dayjs, { Dayjs } from 'dayjs';
import * as z from 'zod';

export const DatetimeSchema = z.codec(
  z.iso.datetime(),
  z.custom<Dayjs>(dayjs.isDayjs),
  {
    decode: (value) => dayjs(value),
    encode: (value) => value.toISOString(),
  },
);
