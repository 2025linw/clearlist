import { render, screen } from '@testing-library/react-native';

import { Provider as ThemeProvider } from '@/context/theme';
import { ThemeMode } from '@/context/theme/types';

import TextInput from '@/components/primitives/text-input';

describe('<TextInput />', () => {
  test.each(['light', 'dark'])(
    'TextInput renders correct value in %s theme',
    async (theme) => {
      await render(
        <ThemeProvider theme={theme as ThemeMode}>
          <TextInput value={'Test'} />
        </ThemeProvider>,
      );

      const input = screen.getByDisplayValue('Test');

      expect(input).toBeOnTheScreen();
    },
  );

  test.each(['light', 'dark'])(
    'TextInput renders correct placeholder in %s theme',
    async (theme) => {
      await render(
        <ThemeProvider theme={theme as ThemeMode}>
          <TextInput placeholder={'Test'} />
        </ThemeProvider>,
      );

      const input = screen.getByPlaceholderText('Test');

      expect(input).toBeOnTheScreen();
    },
  );
});
