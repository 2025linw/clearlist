import { screen } from '@testing-library/react-native';

import { StyleSheet } from 'react-native';

import { buildTheme } from '@contexts/theme/settings';

import Icon from '@components/primitives/icon';

import { renderWithProviders } from '@/test/render';

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

  test('container matches default icon size', async () => {
    await renderWithProviders(
      <Icon
        name="add"
        testID="icon"
      />,
    );

    const icon = screen.getByTestId('icon');
    const iconContainer = icon.parent;
    const style = StyleSheet.flatten(iconContainer?.props.style);

    const componentStyle = theme.components.Icon;

    expect(style.width).toStrictEqual(componentStyle.size);
    expect(style.height).toStrictEqual(componentStyle.size);
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
    const iconContainer = icon.parent;
    const style = StyleSheet.flatten(iconContainer?.props.style);

    expect(icon).toHaveStyle({
      fontSize: 40,
    });
    expect(style.width).toStrictEqual(40);
    expect(style.height).toStrictEqual(40);
  });
});
