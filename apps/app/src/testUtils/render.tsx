import { type RenderOptions, render } from '@testing-library/react-native';

import { type PropsWithChildren, type ReactElement } from 'react';

import ThemeProvider from '@contexts/theme';
import { type ThemeMode } from '@contexts/theme/types';

type TestProvidersProps = PropsWithChildren<{
  theme: ThemeMode;
}>;

function TestProviders({ children, ...props }: TestProvidersProps) {
  return <ThemeProvider theme={props.theme}>{children}</ThemeProvider>;
}

type RenderWithProvidersOptions = Omit<RenderOptions, 'wrapper'> & {
  theme?: ThemeMode;
};

type ProviderProps = PropsWithChildren;

export function renderWithProviders(
  ui: ReactElement,
  { theme = 'light', ...options }: RenderWithProvidersOptions = {},
) {
  function wrapper({ children }: ProviderProps) {
    return <TestProviders theme={theme}>{children}</TestProviders>;
  }

  return render(ui, { ...options, wrapper: wrapper });
}
