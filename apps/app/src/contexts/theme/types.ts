/*
 * This theming was inspired from: https://github.com/tilap/expo-minimal-boilerplate/blob/main/src/contexts/theme/buildTheme.ts
 */
import { type ColorValue } from 'react-native';

import { type colors } from '@contexts/theme/colors';

import { type buildTheme, type variants } from './settings';
import { type typographyVariants } from './tokens';

export type Theme = ReturnType<typeof buildTheme>;

export type ThemeContextType = {
  loaded: boolean;

  theme: Theme;

  themeMode: ThemeMode | 'system';
  setThemeMode: (_: ThemeMode | 'system') => void;
  resetThemeMode: () => void;

  colorTheme: ColorVariantName;
  setColorTheme: (_: ColorVariantName) => void;
  resetColorTheme: () => void;
};

export type Palette = {
  primary: ColorValue;

  background: ColorValue;
  surface: ColorValue;
  border: ColorValue;

  text: ColorValue;
  subtle: ColorValue;

  success: ColorValue;
  info: ColorValue;
  warning: ColorValue;
  error: ColorValue;
  danger: ColorValue;

  colors: typeof colors;
};

export type ThemeMode = 'light' | 'dark';

export type ColorVariantName = keyof typeof variants;

export type ColorScale<T extends string> = {
  [K in `${T}-${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12}`]: ColorValue;
};

export type ThemeColors = {
  primary: ColorScale<'primary'>;
  secondary: ColorScale<'secondary'>;
};

export type ThemeModes = {
  [K in ThemeMode]: ThemeColors;
};

export type TypographyVariants = keyof typeof typographyVariants;
