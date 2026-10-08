import { fireEvent, screen, userEvent } from '@testing-library/react-native';

import { StyleSheet } from 'react-native';

import { buildTheme } from '@contexts/theme/settings';

import { renderWithProviders } from '@/test/render';

import Button from '../button';
import Icon from '../icon';

describe('<Button /> Content', () => {
  test('textOnly renders correctly', async () => {
    await renderWithProviders(<Button>Test</Button>);

    const button = screen.getByRole('button');

    expect(button).toBeVisible();
    expect(button).toHaveTextContent('Test');
  });

  test('iconOnly renders correctly', async () => {
    await renderWithProviders(
      <Button
        icon={
          <Icon
            name="add"
            testID="button-icon"
          />
        }
      />,
    );

    const button = screen.getByRole('button');
    const icon = screen.getByTestId('button-icon');

    expect(button).toBeVisible();
    expect(button).toContainElement(icon);
  });

  test('text and icon renders correctly', async () => {
    await renderWithProviders(
      <Button
        icon={
          <Icon
            name="add"
            testID="button-icon"
          />
        }
      >
        Test
      </Button>,
    );

    const button = screen.getByRole('button');
    const icon = screen.getByTestId('button-icon');

    expect(button).toBeVisible();
    expect(icon).toBeVisible();
    expect(button).toContainElement(icon);
    expect(button).toHaveTextContent('Test', { exact: false });
  });
});

describe('<Button /> Action', () => {
  test('onPress works correctly', async () => {
    const user = userEvent.setup();

    const mockOnPress = jest.fn();
    await renderWithProviders(<Button onPress={mockOnPress}>Click</Button>);

    const button = screen.getByRole('button');

    await user.press(button);

    expect(mockOnPress).toHaveBeenCalledTimes(1);
  });

  test('onPressIn works correctly', async () => {
    const user = userEvent.setup();

    const mockOnPressIn = jest.fn();
    await renderWithProviders(<Button onPressIn={mockOnPressIn}>Click</Button>);

    const button = screen.getByRole('button');

    await user.press(button);

    expect(mockOnPressIn).toHaveBeenCalledTimes(1);
  });

  test('onPressOut works correctly', async () => {
    const user = userEvent.setup();

    const mockOnPressOut = jest.fn();
    await renderWithProviders(
      <Button onPressOut={mockOnPressOut}>Click</Button>,
    );

    const button = screen.getByRole('button');

    await user.press(button);

    expect(mockOnPressOut).toHaveBeenCalledTimes(1);
  });

  test('onLongPress works correctly', async () => {
    const user = userEvent.setup();

    const mockOnPress = jest.fn();
    const mockOnLongPress = jest.fn();
    await renderWithProviders(
      <Button
        onPress={mockOnPress}
        onLongPress={mockOnLongPress}
      >
        Click
      </Button>,
    );

    const button = screen.getByRole('button');

    await user.longPress(button);

    expect(mockOnPress).not.toHaveBeenCalled();
    expect(mockOnLongPress).toHaveBeenCalledTimes(1);
  });

  test('onLongPress with delay works correctly', async () => {
    const user = userEvent.setup();

    const mockOnPress = jest.fn();
    const mockOnLongPress = jest.fn();
    await renderWithProviders(
      <Button
        onPress={mockOnPress}
        onLongPress={mockOnLongPress}
        delayLongPress={750}
      >
        Click
      </Button>,
    );

    const button = screen.getByRole('button');

    await user.longPress(button, { duration: 500 });
    await user.longPress(button, { duration: 750 });

    expect(mockOnPress).toHaveBeenCalledTimes(1);
    expect(mockOnLongPress).toHaveBeenCalledTimes(1);
  });

  test('onHoverIn works correctly', async () => {
    const mockOnHoverIn = jest.fn();
    await renderWithProviders(<Button onHoverIn={mockOnHoverIn}>Click</Button>);

    const button = screen.getByRole('button');

    fireEvent(button, 'onHoverIn');

    expect(mockOnHoverIn).toHaveBeenCalledTimes(1);
  });

  test('onHoverOut works correctly', async () => {
    const mockOnHoverOut = jest.fn();
    await renderWithProviders(
      <Button onHoverOut={mockOnHoverOut}>Click</Button>,
    );

    const button = screen.getByRole('button');

    fireEvent(button, 'onHoverOut');

    expect(mockOnHoverOut).toHaveBeenCalledTimes(1);
  });

  test('disabled prevents interactions', async () => {
    const user = userEvent.setup();

    const mockOnPress = jest.fn();
    const mockOnLongPress = jest.fn();
    await renderWithProviders(
      <Button
        onPress={mockOnPress}
        onLongPress={mockOnLongPress}
        disabled
      >
        Click
      </Button>,
    );

    const button = screen.getByRole('button');

    await user.press(button);
    await user.longPress(button);

    expect(mockOnPress).not.toHaveBeenCalled();
    expect(mockOnLongPress).not.toHaveBeenCalled();
  });
});

describe('<Button /> Style', () => {
  const theme = buildTheme('default', 'light');

  test('icon only button is square', async () => {
    await renderWithProviders(<Button icon={<Icon name="add" />} />);

    const button = screen.getByRole('button');
    const style = StyleSheet.flatten(button.props.style);

    expect(style.width).toEqual(expect.any(Number));
    expect(style.width).toBeGreaterThan(0);
    expect(style.height).toBe(style.width);
  });

  test('default scheme is primary', async () => {
    await renderWithProviders(<Button>Test</Button>);

    const button = screen.getByRole('button');

    const componentStyle = theme.components.Button.scheme.primary;
    const buttonContainerStyle = {
      backgroundColor: componentStyle.backgroundColor,
      borderColor: componentStyle.borderColor,
    };

    expect(button).toHaveStyle(buttonContainerStyle);
  });

  test.each(['primary', 'secondary', 'tertiary', 'success', 'danger'] as const)(
    '%s scheme styles',
    async (schemeName) => {
      await renderWithProviders(<Button scheme={schemeName}>Test</Button>);

      const button = screen.getByRole('button');
      const text = screen.getByText('Test');

      const componentStyle = theme.components.Button.scheme[schemeName];
      const buttonContainerStyle = {
        backgroundColor: componentStyle.backgroundColor,
        borderColor: componentStyle.borderColor,
      };
      const buttonTextStyle = {
        color: componentStyle.color,
      };

      expect(button).toHaveStyle(buttonContainerStyle);
      expect(text).toHaveStyle(buttonTextStyle);
    },
  );

  test('disabled style', async () => {
    await renderWithProviders(<Button disabled>Test</Button>);

    const button = screen.getByRole('button');
    const text = screen.getByText('Test');

    const componentStyle = theme.components.Button.scheme.disabled;
    const buttonContainerStyle = {
      backgroundColor: componentStyle.backgroundColor,
      borderColor: componentStyle.borderColor,
    };
    const buttonTextStyle = {
      color: componentStyle.color,
    };

    expect(button).toHaveStyle(buttonContainerStyle);
    expect(text).toHaveStyle(buttonTextStyle);
  });

  test('hover style', async () => {
    // NOTE: only testing primary

    await renderWithProviders(<Button testOnly_hovered={true}>Test</Button>);

    const button = screen.getByRole('button');

    const componentStyle = theme.components.Button.scheme.primary;
    const buttonHoverStyle = {
      backgroundColor: componentStyle.hovered.backgroundColor,
    };

    expect(button).toHaveStyle(buttonHoverStyle);
  });

  test('press style', async () => {
    // NOTE: only testing primary

    await renderWithProviders(<Button testOnly_pressed={true}>Test</Button>);

    const button = screen.getByRole('button');

    const componentStyle = theme.components.Button.scheme.primary;
    const buttonPressStyle = {
      backgroundColor: componentStyle.pressed.backgroundColor,
    };

    expect(button).toHaveStyle(buttonPressStyle);
  });

  test.each(['primary', 'secondary', 'tertiary'] as const)(
    // NOTE: only testing primary, secondary, and tertiary,
    // as they are the only buttons that would have special handling of borders
    'border prop overrides defaults for %s scheme',
    async (schemeName) => {
      await renderWithProviders(
        <Button
          scheme={schemeName}
          hasBorder
        >
          Test
        </Button>,
      );

      const button = screen.getByRole('button');

      expect(button).toHaveStyle({ borderWidth: 1 });
    },
  );
});
