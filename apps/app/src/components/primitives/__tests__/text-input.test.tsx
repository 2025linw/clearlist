import { screen, userEvent } from '@testing-library/react-native';

import { buildTheme } from '@contexts/theme/settings';

import { renderWithProviders } from '@/test/render';

import TextInput from '../text-input';

describe('<TextInput /> Content', () => {
  test('renders correctly', async () => {
    await renderWithProviders(<TextInput testID="test" />);

    const textInput = screen.getByTestId('test');

    expect(textInput).toBeVisible();
  });

  test('shows value text', async () => {
    await renderWithProviders(<TextInput value="initial input" />);

    const textInput = screen.getByDisplayValue('initial input');

    expect(textInput).toBeVisible();
  });

  test('shows defaultValue text', async () => {
    await renderWithProviders(<TextInput defaultValue="initial input" />);

    const textInput = screen.getByDisplayValue('initial input');

    expect(textInput).toBeVisible();
  });

  test('shows placeholder text', async () => {
    await renderWithProviders(<TextInput placeholder="placeholder test" />);

    const textInput = screen.getByPlaceholderText('placeholder test');

    expect(textInput).toBeVisible();
  });
});

describe('<TextInput /> Style', () => {
  const theme = buildTheme('default', 'light');

  test('style defaults', async () => {
    await renderWithProviders(<TextInput value="initial input" />);

    const textInput = screen.getByDisplayValue('initial input');

    const componentStyle = theme.components.TextInput;

    expect(textInput).toHaveStyle(componentStyle.container);
    expect(textInput).toHaveStyle(componentStyle.input);
    expect(textInput).toHaveProp(
      'placeholderTextColor',
      componentStyle.placeholder.color,
    );
  });

  test('disabled style', async () => {
    await renderWithProviders(
      <TextInput
        value="initial input"
        disabled
      />,
    );

    const textInput = screen.getByDisplayValue('initial input');

    const componentStyle = theme.components.TextInput;

    expect(textInput).toHaveStyle(componentStyle.disabled);
  });

  test('caller can override styles', async () => {
    await renderWithProviders(
      <TextInput
        value="initial input"
        // eslint-disable-next-line react-native/no-inline-styles
        style={{ color: 'orange' }}
      />,
    );

    const textInput = screen.getByDisplayValue('initial input');

    expect(textInput).toHaveStyle({ color: 'orange' });
  });
});

describe('<TextInput /> Action', () => {
  test('typing in text input', async () => {
    const user = userEvent.setup();

    await renderWithProviders(<TextInput testID="test" />);

    const textInput = screen.getByTestId('test');

    await user.type(textInput, 'testing input');

    expect(textInput).toHaveDisplayValue('testing input');
  });

  test('calls onChangeText when typing', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    await renderWithProviders(
      <TextInput
        testID="test"
        onChangeText={onChangeText}
      />,
    );

    const textInput = screen.getByTestId('test');

    await user.type(textInput, 'test');

    expect(onChangeText).toHaveBeenLastCalledWith('test');
  });

  test('editable=false prevents text input', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    await renderWithProviders(
      <TextInput
        editable={false}
        onChangeText={onChangeText}
        testID="test"
      />,
    );

    const textInput = screen.getByTestId('test');

    await user.type(textInput, 'testing');

    expect(textInput).toHaveDisplayValue('');
    expect(onChangeText).not.toHaveBeenCalled();
  });

  test('disabled=true prevents text input', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    await renderWithProviders(
      <TextInput
        disabled={true}
        onChangeText={onChangeText}
        testID="test"
      />,
    );

    const textInput = screen.getByTestId('test');

    await user.type(textInput, 'testing');

    expect(textInput).toHaveDisplayValue('');
    expect(onChangeText).not.toHaveBeenCalled();
  });

  test('disabled=true overrides editable=true', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    await renderWithProviders(
      <TextInput
        editable
        disabled
        onChangeText={onChangeText}
        testID="test"
      />,
    );

    const input = screen.getByTestId('test');

    await user.type(input, 'testing');

    expect(input).toHaveDisplayValue('');
    expect(onChangeText).not.toHaveBeenCalled();
  });
});
