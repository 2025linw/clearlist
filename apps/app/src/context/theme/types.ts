/*
 * This theming was inspired from: https://github.com/tilap/expo-minimal-boilerplate/blob/main/src/contexts/theme/buildTheme.ts
 */
import { ColorValue, Platform } from 'react-native';

export type ThemeContextType = {
  loaded: boolean;

  theme: Theme;

  themeMode: 'system' | ThemeMode;
  setThemeMode: (_: 'system' | ThemeMode) => void;
  resetThemeMode: () => void;

  colorTheme: ColorTheme;
  setColorTheme: (_: ColorTheme) => void;
  resetColorTheme: () => void;
};

export type Palette = {
  primary: ColorValue;
  navigation: ColorValue;

  background: ColorValue;
  surface: ColorValue;
  border: ColorValue;

  text: ColorValue;
  subtle: ColorValue;

  success: ColorValue;

  danger: ColorValue;
};

export type ColorTheme = 'default' | 'pink';
const variants: Record<ColorTheme, Pick<Palette, 'primary' | 'navigation'>> = {
  default: {
    primary: '#2B396D',
    navigation: '#2B396D',
  },
  pink: {
    primary: '#ffd1dc',
    navigation: '#ffd1dc',
  },
};

export type ThemeMode = 'light' | 'dark';
const palettes: Record<ThemeMode, Omit<Palette, 'primary' | 'navigation'>> = {
  light: {
    background: '#E4E4E4',
    surface: '#FAFAFA',
    border: '#D1D1D6',

    text: '#0B0B0B',
    subtle: '#6E6E73',

    success: '#34C759',

    danger: '#EE3333',
  },
  dark: {
    background: '#0B0B0B',
    surface: '#1C1C1E',
    border: '#2C2C2E',

    text: '#E4E4E4',
    subtle: '#8E8E93',

    success: '#30D158',

    danger: '#FF453A',
  },
};

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

export function buildTheme(variant: ColorTheme, darkMode: ThemeMode) {
  const palette: Palette = {
    ...variants[variant],
    ...palettes[darkMode],
  };

  const navigation: ReactNavigation.Theme = {
    dark: darkMode === 'dark',
    colors: {
      primary: palette.primary as string,
      background: palette.navigation as string,
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

  const boxMultiplier = 4;
  const spacings = {
    xs: boxMultiplier / 4,
    sm: boxMultiplier / 2,
    base: boxMultiplier,
    lg: boxMultiplier * 2,
    xl: boxMultiplier * 4,
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
    darkMode: darkMode === 'dark',

    boxMultiplier,

    navigation,

    palette,
    rounded,
    shadows,
    spacings,
    zHeight,

    components: {
      Button: {
        scheme: {
          primary: {
            backgroundColor: palette.primary,
            borderColor: palette.primary,
            color: '#fff',
          },
          default: {
            backgroundColor: palette.surface,
            borderColor: palette.border,
            color: palette.text,
          },
          disabled: {
            backgroundColor: palette.surface,
            borderColor: palette.border,
            color: palette.subtle,
          },
          success: {
            backgroundColor: palette.success,
            borderColor: palette.border,
            color: palette.text,
          },
          danger: {
            backgroundColor: palette.danger,
            borderColor: palette.danger,
            color: '#fff',
          },
        },
        icon: {
          size: typographyVariants.button.fontSize,
        },
      },
      Typography: {
        palette: {
          primary: { color: palette.primary },
          navigation: { color: palette.navigation },

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
