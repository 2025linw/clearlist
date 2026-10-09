import { fireEvent, screen, userEvent } from '@testing-library/react-native';

import { buildTheme } from '@contexts/theme/settings';

import { renderWithProviders } from '@/test/render';

import EditableTypography from '../editable-typography';

describe('<EditableTypography /> Content', () => {
  test('renders correctly', async () => {
    await renderWithProviders(<EditableTypography testID="input" />);

    const textInput = screen.getByTestId('input');

    expect(textInput).toBeVisible();
  });

  test('shows value text in display mode', async () => {
    await renderWithProviders(<EditableTypography value="initial" />);

    const textInput = screen.getByText('initial');

    expect(textInput).toBeVisible();
  });

  test('shows defaultValue text in display mode', async () => {
    await renderWithProviders(<EditableTypography defaultValue="initial" />);

    const textInput = screen.getByText('initial');

    expect(textInput).toBeVisible();
  });

  test('shows placeholder text in display mode', async () => {
    await renderWithProviders(<EditableTypography placeholder="initial" />);

    const textInput = screen.getByText('initial');

    expect(textInput).toBeVisible();
  });

  test('shows value text in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="initial"
        testOnly_editing
      />,
    );

    const textInput = screen.getByDisplayValue('initial');

    expect(textInput).toBeVisible();
  });

  test('shows defaultValue text in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        defaultValue="initial"
        testOnly_editing
      />,
    );

    const textInput = screen.getByDisplayValue('initial');

    expect(textInput).toBeVisible();
  });

  test('shows placeholder text in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        placeholder="initial"
        testOnly_editing
      />,
    );

    const textInput = screen.getByPlaceholderText('initial');

    expect(textInput).toBeVisible();
  });

  test('value takes precedence over defaultValue in display mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="value"
        defaultValue="defaultValue"
      />,
    );

    expect(screen.getByText('value')).toBeVisible();
    expect(screen.queryByText('defaultValue')).toBeNull();
  });

  test('value takes precedence over defaultValue in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="value"
        defaultValue="defaultValue"
        testOnly_editing
      />,
    );

    expect(screen.getByDisplayValue('value')).toBeVisible();
    expect(screen.queryByDisplayValue('defaultValue')).toBeNull();
  });
});

describe('<EditableTypography /> Style', () => {
  const theme = buildTheme('default', 'light');

  test('populated text style defaults in display mode', async () => {
    await renderWithProviders(<EditableTypography value="initial" />);

    const textInput = screen.getByText('initial');

    const componentStyle = theme.components.EditableTypography;
    const style = {
      ...componentStyle.container,
      ...componentStyle.text,
    };

    expect(textInput).toHaveStyle(style);
    expect(textInput).toHaveStyle({ color: theme.palette.text });
  });

  test('placeholder style defaults in display mode', async () => {
    await renderWithProviders(<EditableTypography placeholder="initial" />);

    const textInput = screen.getByText('initial');

    const componentStyle = theme.components.EditableTypography;
    const style = {
      ...componentStyle.container,
      ...componentStyle.text,
    };

    expect(textInput).toHaveStyle(style);
    expect(textInput).toHaveStyle({ color: theme.palette.subtle });
  });

  test('populated text style defaults in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="initial"
        testOnly_editing
      />,
    );

    const textInput = screen.getByDisplayValue('initial');

    const componentStyle = theme.components.EditableTypography;
    const style = {
      ...componentStyle.container,
      ...componentStyle.text,
      ...componentStyle.state.editing,
    };

    expect(textInput).toHaveStyle(style);
    expect(textInput).toHaveStyle({ color: theme.palette.text });
  });

  test('placeholder style defaults in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        placeholder="initial"
        testOnly_editing
      />,
    );

    const textInput = screen.getByPlaceholderText('initial');

    const componentStyle = theme.components.EditableTypography;
    const style = {
      ...componentStyle.container,
      ...componentStyle.text,
      ...componentStyle.state.editing,
    };

    expect(textInput).toHaveStyle(style);
    expect(textInput).toHaveProp('placeholderTextColor', theme.palette.subtle);
  });

  test('disabled style in display mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="initial"
        disabled
      />,
    );

    const componentStyle = theme.components.EditableTypography;

    const textInput = screen.getByText('initial');
    expect(textInput).toHaveStyle(componentStyle.state.disabled);
  });

  test('disabled style in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="initial"
        disabled
        testOnly_editing
      />,
    );

    const componentStyle = theme.components.EditableTypography;

    const textInput = screen.getByDisplayValue('initial');
    expect(textInput).toHaveStyle(componentStyle.state.disabled);
  });

  test('caller can override style in display mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="initial"
        // eslint-disable-next-line react-native/no-inline-styles
        style={{ color: 'orange' }}
      />,
    );

    const textInput = screen.getByText('initial');

    expect(textInput).toHaveStyle({ color: 'orange' });
  });

  test('caller can override style in edit mode', async () => {
    await renderWithProviders(
      <EditableTypography
        value="initial"
        // eslint-disable-next-line react-native/no-inline-styles
        style={{ color: 'orange' }}
        testOnly_editing
      />,
    );

    const textInput = screen.getByDisplayValue('initial');

    expect(textInput).toHaveStyle({ color: 'orange' });
  });
});

describe('<EditableTypography /> Action', () => {
  test('pressing text enters edit mode', async () => {
    const user = userEvent.setup();

    await renderWithProviders(<EditableTypography value="initial" />);

    const text = screen.getByText('initial');

    await user.press(text);

    const textInput = screen.getByDisplayValue('initial');

    expect(textInput).toBeVisible();
    expect(textInput).toHaveProp('autoFocus', true);
  });

  test('typing updates text input', async () => {
    const user = userEvent.setup();

    await renderWithProviders(
      <EditableTypography
        testID="test"
        testOnly_editing
      />,
    );

    const textInput = screen.getByTestId('test');

    await user.type(textInput, 'testing input');

    expect(textInput).toHaveDisplayValue('testing input');
  });

  test('calls onChangeText when typing', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    await renderWithProviders(
      <EditableTypography
        onChangeText={onChangeText}
        testID="test"
        testOnly_editing
      />,
    );

    const textInput = screen.getByTestId('test');

    await user.type(textInput, 'test');

    expect(onChangeText).toHaveBeenLastCalledWith('test');
  });

  test('blur saves text and returns to display mode', async () => {
    const user = userEvent.setup();

    const onSave = jest.fn();
    await renderWithProviders(
      <EditableTypography
        placeholder="Enter text"
        onSave={onSave}
      />,
    );

    const text = screen.getByText('Enter text');

    await user.press(text);

    const textInput = screen.getByPlaceholderText('Enter text');

    await user.type(textInput, 'testing input', {
      skipBlur: true,
    });

    expect(onSave).not.toHaveBeenCalled();

    fireEvent(textInput, 'blur');

    expect(onSave).toHaveBeenCalledTimes(1);
    expect(onSave).toHaveBeenCalledWith('testing input');

    expect(await screen.findByText('testing input')).toBeVisible();
    expect(screen.queryByDisplayValue('testing input')).toBeNull();
  });

  test('clearing text saves an empty string and shows placeholder', async () => {
    const user = userEvent.setup();

    const onSave = jest.fn();

    await renderWithProviders(
      <EditableTypography
        defaultValue="initial"
        placeholder="Enter text"
        onSave={onSave}
      />,
    );

    const text = screen.getByText('initial');

    await user.press(text);

    const textInput = screen.getByDisplayValue('initial');

    await user.clear(textInput);

    expect(textInput).toHaveDisplayValue('');

    fireEvent(textInput, 'blur');

    expect(onSave).toHaveBeenCalledTimes(1);
    expect(onSave).toHaveBeenCalledWith('');

    expect(await screen.findByText('Enter text')).toBeVisible();
    expect(screen.queryByPlaceholderText('Enter text')).toBeNull();
  });

  test('editable=false prevents entering edit mode', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    const onSave = jest.fn();

    await renderWithProviders(
      <EditableTypography
        value="initial"
        editable={false}
        onChangeText={onChangeText}
        onSave={onSave}
      />,
    );

    const text = screen.getByText('initial');

    await user.press(text);

    expect(text).toBeVisible();
    expect(screen.queryByDisplayValue('initial')).toBeNull();
    expect(onChangeText).not.toHaveBeenCalled();
    expect(onSave).not.toHaveBeenCalled();
  });

  test('disabled=true prevents entering edit mode', async () => {
    const user = userEvent.setup();

    const onChangeText = jest.fn();
    const onSave = jest.fn();
    await renderWithProviders(
      <EditableTypography
        value="initial"
        disabled
        onChangeText={onChangeText}
        onSave={onSave}
      />,
    );

    const text = screen.getByText('initial');

    await user.press(text);

    expect(text).toBeVisible();
    expect(screen.queryByDisplayValue('initial')).toBeNull();
    expect(onChangeText).not.toHaveBeenCalled();
    expect(onSave).not.toHaveBeenCalled();
  });
});
