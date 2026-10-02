import { colors } from './colors';
import { type Palette, type ThemeColors } from './types';

export function buildPalette(themeColors: ThemeColors): Palette {
  return {
    primary: themeColors.primary['primary-9'],

    background: themeColors.secondary['secondary-1'],
    surface: themeColors.secondary['secondary-2'],
    border: themeColors.secondary['secondary-6'],

    text: themeColors.secondary['secondary-12'],
    subtle: themeColors.secondary['secondary-11'],

    success: colors.green['green-9'],
    info: colors.blue['blue-9'],
    warning: colors.yellow['yellow-9'],
    danger: colors.red['red-9'],
    error: colors.red['red-9'],
  };
}
