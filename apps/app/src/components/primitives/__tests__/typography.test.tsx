import { screen } from '@testing-library/react-native';

import { buildTheme } from '@contexts/theme/settings';

import { renderWithProviders } from '@/test/render';

import Typography from '../typography';

describe('<Typography /> Content', () => {
  test('renders correctly', async () => {
    await renderWithProviders(<Typography>Test</Typography>);

    const typography = screen.getByText('Test');

    expect(typography).toBeVisible();
  });
});

describe('<Typography /> Style', () => {
  const theme = buildTheme('default', 'light');

  test('default variant is text', async () => {
    await renderWithProviders(<Typography>Test</Typography>);

    const typography = screen.getByText('Test');

    expect(typography).toHaveStyle(theme.components.Typography.variants.text);
  });

  test.each(['h1', 'h2', 'h3', 'h4', 'text', 'button'] as const)(
    '%s variant styles',
    async (variantName) => {
      await renderWithProviders(
        <Typography variant={variantName}>Test</Typography>,
      );

      const typography = screen.getByText('Test');

      expect(typography).toHaveStyle(
        theme.components.Typography.variants[variantName],
      );
    },
  );

  test.each(['primary', 'text', 'subtle', 'success', 'danger'] as const)(
    '%s palette styles',
    async (paletteName) => {
      await renderWithProviders(
        <Typography palette={paletteName}>Test</Typography>,
      );

      const typography = screen.getByText('Test');

      expect(typography).toHaveStyle(
        theme.components.Typography.palette[paletteName],
      );
    },
  );
});
