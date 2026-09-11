/*
 * This theming was inspired from: https://github.com/tilap/expo-minimal-boilerplate/blob/main/src/contexts/theme/buildTheme.ts
 */
import color from 'color';

import { ColorValue, Platform } from 'react-native';

import { Colors } from '@/context/theme/colors/types';

import { ColorVariant, colors, main } from './colors';
import { spacings } from './spacing';

export type ThemeContextType = {
  loaded: boolean;

  theme: Theme;

  themeMode: 'system' | ThemeMode;
  setThemeMode: (_: 'system' | ThemeMode) => void;
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
};

export type ColorVariantName = 'default';
const variants: Record<ColorVariantName, ColorVariant> = {
  default: main.colors,
};

export type ThemeMode = 'light' | 'dark';
function buildPalette(themeColors: Colors): Palette {
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

const typographyVariants = {
  h1: {
    fontFamily: 'Inter-Black',
    fontSize: 24,
  },
  h2: {
    fontFamily: 'Inter-Bold',
    fontSize: 20,
  },
  h3: {
    fontFamily: 'Inter-Bold',
    fontSize: 18,
  },
  h4: {
    fontFamily: 'Inter-Regular',
    fontSize: 13,
    textTransform: 'uppercase',
  },
  text: {
    fontFamily: 'Inter-Regular',
    fontSize: 16,
  },
  button: {
    fontFamily: 'Inter-Bold',
    fontSize: 18,
  },
};
export type TypographyVariants = keyof typeof typographyVariants;

const shadows = {
  low: {
    ...Platform.select({
      ios: {
        shadowColor: '#000',
        shadowOffset: { width: 0, height: 2 },
        shadowOpacity: 0.15,
        shadowRadius: 1.2,
      },
      android: { elevation: 3 },
    }),
  },
  base: {
    ...Platform.select({
      ios: {
        shadowColor: '#000',
        shadowOffset: { width: 0, height: 3 },
        shadowOpacity: 0.23,
        shadowRadius: 3.85,
      },
      android: { elevation: 6 },
    }),
  },
  high: {
    ...Platform.select({
      ios: {
        shadowColor: '#000',
        shadowOffset: { width: 0, height: 5 },
        shadowOpacity: 0.38,
        shadowRadius: 6.37,
      },
      android: { elevation: 10 },
    }),
  },
};

export function buildTheme(variant: ColorVariantName, theme: ThemeMode) {
  const themeColors = variants[variant][theme];
  const palette: Palette = buildPalette(themeColors);

  const navigation: ReactNavigation.Theme = {
    dark: theme === 'dark',
    colors: {
      primary: palette.primary as string,
      background: palette.primary as string,
      card: palette.surface as string,
      text: palette.text as string,
      border: palette.surface as string,
      notification: 'rgb(255, 59, 48)',
    },
    fonts: {
      regular: {
        fontFamily: 'Inter-Regular',
        fontWeight: '400',
      },
      medium: {
        fontFamily: 'Inter-Medium',
        fontWeight: '500',
      },
      bold: {
        fontFamily: 'Inter-Bold',
        fontWeight: '700',
      },
      heavy: {
        fontFamily: 'Inter-Black',
        fontWeight: '900',
      },
    },
  };

  const rounded = {
    sm: 3,
    base: 6,
    lg: 10,
    full: 9999,
  };

  const zHeight = {
    base: 0,

    content: 1,
    floating: 10,

    overlay: 100,
    modal: 1000,

    toast: 1100,
    tooltip: 1200,

    max: 9999,
  };

  return {
    darkMode: theme === 'dark',

    navigation,

    palette,
    rounded,
    shadows,
    spacings: spacings,
    zHeight,

    components: {
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
            borderColor: 'transparent',
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
          // default: {
          //   backgroundColor: palette.surface,
          //   borderColor: palette.border,
          //   color: palette.text,
          // },
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
        },
        icon: {
          size: typographyVariants.button.fontSize + 4,
        },
      },
      Typography: {
        palette: {
          primary: { color: palette.primary },
          navigation: { color: palette.primary },

          text: { color: palette.text },
          subtle: { color: palette.subtle },

          danger: { color: palette.danger },
        },
        variants: typographyVariants,
      },
      TextInput: {
        input: { color: palette.text },
        placeholder: { color: palette.subtle },
      },
      Icon: {
        size: typographyVariants.button.fontSize,
        color: palette.text,
      },
    },
  } as const;
}
export type Theme = ReturnType<typeof buildTheme>;
