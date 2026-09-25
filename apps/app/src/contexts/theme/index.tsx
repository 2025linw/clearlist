import { type PropsWithChildren, createContext, useContext } from 'react';
import { useColorScheme } from 'react-native';

import usePersisted from '@hooks/use-persisted';

import {
  type ColorVariantName,
  type ThemeContextType,
  type ThemeMode,
} from './types';
import { buildTheme } from './types';

const ThemeContext = createContext<ThemeContextType>(
  {} as unknown as ThemeContextType,
); // TODO: fix this jank?

type ProviderProps = PropsWithChildren<{
  theme?: ThemeMode;
  onThemeVariantChange?: (_v: ColorVariantName) => void; // TODO: is this needed?
}>;

export function Provider({ children, ...props }: ProviderProps) {
  const {
    value: themeMode,
    setValue: _setThemeMode,
    loaded: themeLoaded,
  } = usePersisted('systemTheme');
  const {
    value: colorTheme,
    setValue: _setColorTheme,
    loaded: colorLoaded,
  } = usePersisted('colorTheme');

  const systemTheme = useColorScheme();

  const darkMode =
    themeMode === 'system'
      ? systemTheme === 'dark'
        ? 'dark'
        : 'light'
      : themeMode;

  const theme = buildTheme(colorTheme, props.theme ?? darkMode);

  function setThemeMode(v: 'system' | ThemeMode) {
    _setThemeMode(v);
  }
  function resetThemeMode() {
    _setThemeMode('system');
  }

  function setColorTheme(v: ColorVariantName) {
    _setColorTheme(v);
    props.onThemeVariantChange?.(v);
  }
  function resetColorTheme() {
    _setColorTheme('default');
    props.onThemeVariantChange?.('default');
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
      }}
    >
      {children}
    </ThemeContext>
  );
}

export function useThemeContext() {
  const ctx = useContext(ThemeContext);
  if (!ctx) {
    throw new Error('useThemeContext must be used inside Theme Provider');
  }

  return ctx;
}

export function useTheme() {
  try {
    const { theme } = useThemeContext();

    return theme;
  } catch {
    throw new Error('useTheme must be used inside Theme Provider');
  }
}

export function useColorTheme() {
  try {
    const { colorTheme, setColorTheme } = useThemeContext();

    return [colorTheme, setColorTheme] as const;
  } catch {
    throw new Error('useColorTheme must be used inside Theme Provider');
  }
}

export function useResetColorTheme() {
  try {
    const { resetColorTheme } = useThemeContext();

    return resetColorTheme;
  } catch {
    throw new Error('useResetColorTheme must be used inside Theme Provider');
  }
}

export function useThemeMode() {
  try {
    const { themeMode, setThemeMode } = useThemeContext();

    return [themeMode, setThemeMode] as const;
  } catch {
    throw new Error('useThemeMode must be used inside Theme Provider');
  }
}

export function useResetThemeMode() {
  try {
    const { resetThemeMode } = useThemeContext();

    return resetThemeMode;
  } catch {
    throw new Error('useResetThemeMode must be used inside Theme Provider');
  }
}
