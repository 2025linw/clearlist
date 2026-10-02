import color from 'color';

import { buildPalette } from './build-palette';
import { brand } from './colors';
import {
  rounded,
  shadows,
  spacings,
  typographyVariants,
  zHeight,
} from './tokens';
import { ColorVariantName, Palette, ThemeMode } from './types';

export const variants = {
  default: brand,
} as const;

export function buildTheme(variant: ColorVariantName, theme: ThemeMode) {
  const themeColors = variants[variant][theme];
  const palette: Palette = buildPalette(themeColors);

  return {
    darkMode: theme === 'dark',

    palette,
    spacings,
    rounded,
    shadows,
    zHeight,

    components: {
      // TODO: all primitives to this theme, also add toast
      Typography: {
        palette: {
          primary: { color: palette.primary },

          text: { color: palette.text },
          subtle: { color: palette.subtle },

          danger: { color: palette.danger },
        },
        variants: typographyVariants,
      },
      Icon: {
        size: typographyVariants.text.fontSize,
        color: palette.text,
      },
      Button: {
        scheme: {
          primary: {
            backgroundColor: palette.primary,
            borderColor: 'transparent',
            color: '#fff',

            hovered: {
              backgroundColor: themeColors.primary['primary-10'],
            },
            pressed: {
              backgroundColor: color(themeColors.primary['primary-10'])
                .darken(0.08)
                .saturate(0.1)
                .hex(),
            },
          },
          secondary: {
            backgroundColor: themeColors.primary['primary-3'],
            borderColor: palette.primary,
            color: palette.primary,

            hovered: {
              backgroundColor: themeColors.primary['primary-4'],
            },
            pressed: {
              backgroundColor: themeColors.primary['primary-5'],
            },
          },
          tertiary: {
            backgroundColor: 'transparent',
            borderColor: palette.primary,
            color: palette.primary,

            hovered: {
              backgroundColor: themeColors.primary['primary-3'],
            },
            pressed: {
              backgroundColor: color(themeColors.primary['primary-4'])
                .darken(0.08)
                .saturate(0.1)
                .hex(),
            },
          },
          success: {
            backgroundColor: palette.success,
            borderColor: palette.border,
            color: palette.text,

            hovered: {
              backgroundColor: themeColors.primary['primary-10'],
            },
            pressed: {
              backgroundColor: color(themeColors.primary['primary-10'])
                .darken(0.08)
                .saturate(0.1)
                .hex(),
            },
          },
          danger: {
            backgroundColor: palette.danger,
            borderColor: palette.danger,
            color: '#fff',

            hovered: {
              backgroundColor: themeColors.primary['primary-10'],
            },
            pressed: {
              backgroundColor: color(palette.danger)
                .darken(0.08)
                .saturate(0.1)
                .hex(),
            },
          },
          disabled: {
            backgroundColor: palette.surface,
            borderColor: palette.border,
            color: palette.subtle,

            hovered: {
              backgroundColor: themeColors.primary['primary-10'],
            },
            pressed: {
              backgroundColor: color(themeColors.primary['primary-10'])
                .darken(0.08)
                .saturate(0.1)
                .hex(),
            },
          },
        },
        icon: {
          size: typographyVariants.button.fontSize + 4,
        },
      },
      TextInput: {
        input: { color: palette.text },
        placeholder: { color: palette.subtle },
      },
    },
  } as const;
}
