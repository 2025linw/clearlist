import { screen, userEvent } from '@testing-library/react-native';

import { buildTheme } from '@contexts/theme/settings';

import { renderWithProviders } from '@/test/render';

import Checkbox from '../checkbox';

describe('<Checkbox /> Content', () => {
  test('renders correctly', async () => {
    await renderWithProviders(<Checkbox />);

    const checkbox = screen.getByRole('checkbox');

    expect(checkbox).toBeVisible();
  });
});

describe('<Checkbox /> Style', () => {
  const theme = buildTheme('default', 'light');

  test.each([
    { disabled: false, colorKey: 'text' },
    { disabled: true, colorKey: 'subtle' },
  ] as const)(
    'uses $colorKey color when disabled=$disabled',
    async ({ disabled, colorKey }) => {
      await renderWithProviders(
        <Checkbox
          disabled={disabled}
          testID="test"
        />,
      );

      const checkboxIcon = screen.getByTestId('test-icon');

      expect(checkboxIcon).toHaveStyle({ color: theme.palette[colorKey] });
    },
  );
});

describe('<Checkbox /> Action', () => {
  test('works correctly', async () => {
    const user = userEvent.setup();

    const mockOnChange = jest.fn((state) => state);
    await renderWithProviders(<Checkbox onChange={mockOnChange} />);

    const checkbox = screen.getByRole('checkbox');

    await user.press(checkbox);
    await user.press(checkbox);

    expect(mockOnChange).toHaveBeenCalledTimes(2);
    expect(mockOnChange.mock.calls[0][0]).toStrictEqual(true);
    expect(mockOnChange.mock.calls[1][0]).toStrictEqual(false);
  });

  test('disabled prevents interactions', async () => {
    const user = userEvent.setup();

    const mockOnChange = jest.fn((state) => state);
    await renderWithProviders(
      <Checkbox
        onChange={mockOnChange}
        disabled
      />,
    );

    const checkbox = screen.getByRole('checkbox');

    await user.press(checkbox);
    await user.press(checkbox);

    expect(mockOnChange).not.toHaveBeenCalled();
  });
});
