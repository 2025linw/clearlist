/* eslint-disable react-native/no-inline-styles */
import { render, screen } from '@testing-library/react-native';

import { Provider as ThemeProvider } from '@/context/theme';
import { ThemeMode } from '@/context/theme/types';

import Typography from '@/components/primitives/typography';

describe('<Typography />', () => {
  test.each(['light', 'dark'])(
    'Typography renders correct text in %s theme',
    async (theme) => {
      await render(
        <ThemeProvider theme={theme as ThemeMode}>
          <Typography>Test</Typography>
        </ThemeProvider>,
      );

      const text = screen.getByText('Test');

      expect(text).toBeOnTheScreen();
    },
  );

  test('Typography uses correct light theme color', async () => {
    await render(
      <ThemeProvider theme="light">
        <Typography>Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#0B0B0B',
    });
  });

  test('Typography uses correct dark theme color', async () => {
    await render(
      <ThemeProvider theme="dark">
        <Typography>Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#E4E4E4',
    });
  });

  test('Typography default palette and variant renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography>Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#0B0B0B',
      fontFamily: 'Inter-Regular',
      fontSize: 16,
    });
  });

  test('Typography palette primary renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography palette="primary">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#2B396D',
    });
  });

  test('Typography palette navigation renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography palette="navigation">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#2B396D',
    });
  });

  test('Typography palette text renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography palette="text">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#0B0B0B',
    });
  });

  test('Typography palette subtle renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography palette="subtle">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#6E6E73',
    });
  });

  test('Typography palette danger renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography palette="danger">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: '#EE3333',
    });
  });

  test('Typography variant h1 renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography variant="h1">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      fontFamily: 'Inter-Black',
      fontSize: 24,
    });
  });

  test('Typography variant h2 renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography variant="h2">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      fontFamily: 'Inter-Bold',
      fontSize: 20,
    });
  });

  test('Typography variant h3 renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography variant="h3">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      fontFamily: 'Inter-Bold',
      fontSize: 18,
    });
  });

  test('Typography variant h4 renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography variant="h4">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      fontFamily: 'Inter-Regular',
      fontSize: 13,
      textTransform: 'uppercase',
    });
  });

  test('Typography variant text renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography variant="text">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      fontFamily: 'Inter-Regular',
      fontSize: 16,
    });
  });

  test('Typography variant button renders correctly', async () => {
    await render(
      <ThemeProvider>
        <Typography variant="button">Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      fontFamily: 'Inter-Bold',
      fontSize: 18,
    });
  });

  test('Typography allows styles', async () => {
    await render(
      <ThemeProvider>
        <Typography style={{ color: 'red' }}>Test</Typography>
      </ThemeProvider>,
    );

    const text = screen.getByText('Test');

    expect(text).toHaveStyle({
      color: 'red',
    });
  });
});
