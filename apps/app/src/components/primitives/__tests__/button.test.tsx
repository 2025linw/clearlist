import { fireEvent, screen, userEvent } from '@testing-library/react-native';

import { renderWithProviders } from '@/test/render';

import Button from '../button';
import Icon from '../icon';

describe('<Button /> Visual Content', () => {
  test('Text renders correctly', async () => {
    await renderWithProviders(<Button>Click Here</Button>);

    const button = screen.getByRole('button');

    expect(button).toBeOnTheScreen();
    expect(button).toHaveTextContent('Click Here');
  });

  test('Icon renders correctly', async () => {
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

    expect(button).toBeOnTheScreen();
    expect(button).toContainElement(icon);
  });
});

describe('<Button /> Action', () => {
  test('onPress works correctly', async () => {
    const user = userEvent.setup();
    const mockOnPress = jest.fn();

    await renderWithProviders(<Button onPress={mockOnPress}>Click</Button>);

    const button = screen.getByText('Click');

    await user.press(button);

    expect(mockOnPress).toHaveBeenCalledTimes(1);
  });

  test('onPressIn works correctly', async () => {
    const user = userEvent.setup();
    const mockOnPressIn = jest.fn();

    await renderWithProviders(<Button onPressIn={mockOnPressIn}>Click</Button>);

    const button = screen.getByText('Click');

    await user.press(button);

    expect(mockOnPressIn).toHaveBeenCalledTimes(1);
  });

  test('onPressOut works correctly', async () => {
    const user = userEvent.setup();
    const mockOnPressOut = jest.fn();

    await renderWithProviders(
      <Button onPressOut={mockOnPressOut}>Click</Button>,
    );

    const button = screen.getByText('Click');

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

    const button = screen.getByText('Click');

    await user.longPress(button);

    expect(mockOnPress).toHaveBeenCalledTimes(0);
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

    const button = screen.getByText('Click');

    await user.longPress(button, { duration: 500 });
    await user.longPress(button, { duration: 750 });

    expect(mockOnPress).toHaveBeenCalledTimes(1);
    expect(mockOnLongPress).toHaveBeenCalledTimes(1);
  });

  test('onHoverIn works correctly', async () => {
    const mockOnHoverIn = jest.fn();

    await renderWithProviders(<Button onHoverIn={mockOnHoverIn}>Click</Button>);

    const button = screen.getByText('Click');

    fireEvent(button, 'onHoverIn');

    expect(mockOnHoverIn).toHaveBeenCalledTimes(1);
  });

  test('onHoverOut works correctly', async () => {
    const mockOnHoverOut = jest.fn();

    await renderWithProviders(
      <Button onHoverOut={mockOnHoverOut}>Click</Button>,
    );

    const button = screen.getByText('Click');

    fireEvent(button, 'onHoverOut');

    expect(mockOnHoverOut).toHaveBeenCalledTimes(1);
  });
});

describe('<Button /> Style', () => {
  test('Default style (primary)', async () => {
    // TODO
  });

  test('secondary style', async () => {
    // TODO
  });

  test('tertiary style', async () => {
    // TODO
  });

  test('success style', async () => {
    // TODO
  });

  test('danger style', async () => {
    // TODO
  });

  test('disabled style', async () => {
    // TODO
  });
});
