import { ColorValue } from 'react-native';

import type { ThemeMode } from '../types';

export type ColorScale<T extends string> = {
  [K in `${T}-${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12}`]: ColorValue;
};

export type Colors = {
  primary: ColorScale<'primary'>;
  secondary: ColorScale<'secondary'>;
};

export type ColorVariant = {
  [K in ThemeMode]: Colors;
};
