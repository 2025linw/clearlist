import { type RenderOptions, render } from '@testing-library/react-native';

import { type PropsWithChildren, type ReactElement } from 'react';

import { Provider as ThemeProvider } from '@contexts/theme';
import { type ThemeMode } from '@contexts/theme/types';

type TestProvidersProps = PropsWithChildren & {
  theme: ThemeMode;
};

function Providers({ children, ...props }: TestProvidersProps) {
  return <ThemeProvider theme={props.theme}>{children}</ThemeProvider>;
}

type RenderWithProvidersOptions = Omit<RenderOptions, 'wrapper'> & {
  theme?: ThemeMode;
};

export function renderWithProviders(
  ui: ReactElement,
  { theme = 'light', ...options }: RenderWithProvidersOptions = {},
) {
  function wrapper({ children }: PropsWithChildren) {
    return <Providers theme={theme}>{children}</Providers>;
  }

  return render(ui, { ...options, wrapper: wrapper });
}
