import { type PropsWithChildren, createContext, useContext } from 'react';
import { useColorScheme } from 'react-native';

import { getColorThemes } from '@contexts/theme/colors';
import { usePersisted } from '@hooks/use-persisted';

import { buildTheme } from './settings';
import {
  type ColorVariantName,
  type ThemeContextType,
  type ThemeMode,
} from './types';

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

type ThemeProviderProps = PropsWithChildren<{
  theme?: ThemeMode;
}>;

export default function ThemeProvider({
  children,
  ...props
}: ThemeProviderProps) {
  const {
    value: themeMode,
    setValue: _setThemeMode,
    loaded: themeLoaded,
  } = usePersisted('themeMode');
  const {
    value: colorTheme,
    setValue: _setColorTheme,
    loaded: colorLoaded,
  } = usePersisted('colorTheme');

  const systemTheme = useColorScheme();
  const resolvedThemeMode =
    themeMode === 'system'
      ? systemTheme === 'dark'
        ? 'dark'
        : 'light'
      : themeMode;

  const theme = buildTheme(colorTheme, props.theme ?? resolvedThemeMode);

  function setThemeMode(v: ThemeMode | 'system') {
    _setThemeMode(v);
  }
  function resetThemeMode() {
    _setThemeMode('system');
  }

  function setColorTheme(v: ColorVariantName) {
    _setColorTheme(v);
  }
  function resetColorTheme() {
    _setColorTheme('default');
  }
  function getAllColorThemes() {
    return getColorThemes(resolvedThemeMode);
  }

  const loaded = themeLoaded && colorLoaded;

  return (
    <ThemeContext
      value={{
        loaded,
        theme,
        themeMode,
        setThemeMode,
        resetThemeMode,
        colorTheme,
        setColorTheme,
        resetColorTheme,
        getAllColorThemes,
      }}
    >
      {children}
    </ThemeContext>
  );
}

export function useThemeContext() {
  const ctx = useContext(ThemeContext);
  if (!ctx) {
    throw new Error('Theme Context must be used inside Theme Provider');
  }

  return ctx;
}

export function useTheme() {
  const { theme } = useThemeContext();

  return theme;
}

export function useColorTheme() {
  const { colorTheme, setColorTheme } = useThemeContext();

  return [colorTheme, setColorTheme] as const;
}

export function useResetColorTheme() {
  const { resetColorTheme } = useThemeContext();

  return resetColorTheme;
}

export function useThemeMode() {
  const { themeMode, setThemeMode } = useThemeContext();

  return [themeMode, setThemeMode] as const;
}

export function useResetThemeMode() {
  const { resetThemeMode } = useThemeContext();

  return resetThemeMode;
}
