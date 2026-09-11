import { ReactElement, cloneElement } from 'react';
import { Pressable, PressableProps, StyleSheet, View } from 'react-native';

import { useTheme, useThemeMode } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Icon, { IconProps } from '@/components/icon';
import Typography from '@/components/primitives/typography';

type ButtonSchemes = keyof Omit<
  Theme['components']['Button']['scheme'],
  'disabled'
>;

export type ButtonProps = PressableProps & {
  icon?: ReactElement<IconProps>;
  children?: string;
  scheme?: ButtonSchemes;
};

export default function Button({
  scheme = 'primary',
  children,
  disabled,
  style,
  ...props
}: ButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(theme, disabled ? 'disabled' : scheme);

  let icon = undefined;
  if (props.icon) {
    icon = cloneElement(props.icon, {
      size: props.icon.props.size ?? styles.icon.fontSize,
      color: props.icon.props.color ?? styles.typography.color,
    });
  }

  return (
    <Pressable
      {...props}
      disabled={disabled}
      style={(state) => [
        styles.container,
        state.hovered && styles.hoveredStyle,
        state.pressed && styles.pressedStyle,
        typeof style === 'function' ? style(state) : style,
      ]}
    >
      {icon}

      {children && (
        <Typography
          variant="button"
          style={styles.typography}
          selectable={false}
        >
          {children}
        </Typography>
      )}
    </Pressable>
  );
}

function buildStyles(theme: Theme, scheme: ButtonSchemes | 'disabled') {
  const componentStyle = theme.components.Button;

  const schemeStyle = componentStyle.scheme[scheme];

  return StyleSheet.create({
    container: {
      padding: theme.spacings.x2,

      flexDirection: 'row',
      alignItems: 'center',
      gap: theme.spacings.x2,

      backgroundColor: schemeStyle.backgroundColor,
      borderWidth: 1,
      borderRadius: theme.rounded.base,
      borderColor: schemeStyle.borderColor,
    },
    hoveredStyle: {
      backgroundColor: schemeStyle.hovered.backgroundColor,
    },
    pressedStyle: {
      backgroundColor: schemeStyle.pressed.backgroundColor,
    },
    typography: {
      color: componentStyle.scheme[scheme].color,
      userSelect: 'none',
    },
    icon: {
      fontSize: componentStyle.icon.size,
    },
  });
}

export function Demo() {
  const [themeMode, setThemeMode] = useThemeMode();

  return (
    /* eslint-disable react-native/no-inline-styles */
    <View
      style={[StyleSheet.absoluteFill, { gap: 16, alignItems: 'flex-start' }]}
    >
      <Button>Default</Button>
      <Button scheme="primary">Primary</Button>
      <Button scheme="secondary">Secondary</Button>
      <Button scheme="tertiary">Tertiary</Button>
      <Button scheme="danger">Danger</Button>
      <Button disabled>Disabled</Button>
      <Button icon={<Icon name="home-outline" />} />

      <Button
        style={{ position: 'absolute', bottom: 15 }}
        onPress={() => setThemeMode(themeMode === 'light' ? 'dark' : 'light')}
      >
        Toggle Theme
      </Button>
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
