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
import { type ColorVariantName, type Palette, type ThemeMode } from './types';

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
          text: { color: palette.text },
          subtle: { color: palette.subtle },

          primary: { color: palette.primary },

          success: { color: palette.success },
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
            color: '#fff',

            hovered: {
              backgroundColor: palette.colors.green['green-10'],
            },
            pressed: {
              backgroundColor: color(palette.colors.green['green-10'])
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
              backgroundColor: palette.colors.red['red-10'],
            },
            pressed: {
              backgroundColor: color(palette.colors.red['red-10'])
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
              backgroundColor: palette.surface,
            },
            pressed: {
              backgroundColor: palette.surface,
            },
          },
        },
        icon: {
          size: typographyVariants.button.fontSize + 4,
        },
      },
      TextInput: {
        container: {
          borderWidth: spacings.thin,
          borderColor: palette.border,
          borderRadius: rounded.base,
          padding: spacings.x2,
        },
        disabled: {
          backgroundColor: themeColors.secondary['secondary-3'],
        },
        input: {
          ...typographyVariants.text,
          color: palette.text,
        },
        placeholder: { color: palette.subtle },
      },
      EditableTypography: {
        container: {
          borderWidth: spacings.thin,
          borderColor: 'transparent',
          borderRadius: rounded.base,
          padding: spacings.x2,
        },
        state: {
          editing: {
            borderColor: palette.border,
          },
          disabled: {
            backgroundColor: themeColors.secondary['secondary-3'],
          },
        },
        text: typographyVariants.text,
      },
      Toast: {
        size: typographyVariants.button.fontSize,
        palette: {
          default: {
            color: palette.text,
          },
          success: {
            color: palette.success,
          },
          warn: {
            color: palette.warning,
          },
          error: {
            color: palette.error,
          },
        },
      },
    },
  } as const;
}
