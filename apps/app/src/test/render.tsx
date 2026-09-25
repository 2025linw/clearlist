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

export function renderWithProviders(
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>,
) {
  return render(ui, { wrapper: Providers, ...options });
}
