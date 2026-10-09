import { screen } from '@testing-library/react-native';

import { StyleSheet } from 'react-native';

import { buildTheme } from '@contexts/theme/settings';

import { renderWithProviders } from '@/test/render';

import Icon from '../icon';

describe('<Icon /> Content', () => {
  test('renders correctly', async () => {
    await renderWithProviders(
      <Icon
        name="add"
        testID="icon"
      />,
    );

    const icon = screen.getByTestId('icon');

    expect(icon).toBeVisible();
  });
});

describe('<Icon /> Style', () => {
  const theme = buildTheme('default', 'light');

  test('default style', async () => {
    await renderWithProviders(
      <Icon
        name="add"
        testID="icon"
      />,
    );

    const icon = screen.getByTestId('icon');

    const componentStyle = theme.components.Icon;

    expect(icon).toHaveStyle({
      color: componentStyle.color,
      fontSize: componentStyle.size,
    });
  });

  test('color prop overrides default', async () => {
    await renderWithProviders(
      <Icon
        name="add"
        color="pink"
        testID="icon"
      />,
    );

    const icon = screen.getByTestId('icon');

    expect(icon).toHaveStyle({
      color: 'pink',
    });
  });

  test('size prop overrides default', async () => {
    await renderWithProviders(
      <Icon
        name="add"
        size={40}
        testID="icon"
      />,
    );

    const icon = screen.getByTestId('icon');
    const style = StyleSheet.flatten(icon.props.style);

    expect(icon).toHaveStyle({
      fontSize: 40,
    });
    expect(style.width).toStrictEqual(40);
    expect(style.height).toStrictEqual(40);
  });

  test('caller can override style', async () => {
    await renderWithProviders(
      <Icon
        name="add"
        // eslint-disable-next-line react-native/no-inline-styles
        style={{ backgroundColor: 'red' }}
        testID="icon"
      />,
    );

    const icon = screen.getByTestId('icon');

    expect(icon).toHaveStyle({ backgroundColor: 'red' });
  });
});
