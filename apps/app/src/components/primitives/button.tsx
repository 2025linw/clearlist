import { type ReactElement } from 'react';
import { cloneElement } from 'react';
import {
  type ColorValue,
  type PressableProps,
  type StyleProp,
  type TextStyle,
} from 'react-native';
import { Pressable, StyleSheet, View } from 'react-native';

import { useTheme, useThemeMode } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import Icon, { type IconProps } from '@components/primitives/icon';
import Typography from '@components/primitives/typography';

import { type ElementProps } from './types';

type ButtonSchemes = keyof Omit<
  Theme['components']['Button']['scheme'],
  'disabled'
>;

type ButtonContent =
  | {
      children: string;
      icon?: ReactElement<IconProps>;
    }
  | {
      children?: never;
      icon: ReactElement<IconProps>;
    };

export type ButtonProps = Omit<PressableProps, 'children' | ElementProps> &
  ButtonContent & {
    scheme?: ButtonSchemes;
    hasBorder?: boolean;
    rounded?: boolean;
  } & {
    typographyStyle?: StyleProp<TextStyle>;
  };

export default function Button({
  scheme = 'primary',
  children,
  disabled,
  style,
  ...props
}: ButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(
    theme,
    disabled ? 'disabled' : scheme,
    props.rounded,
    props.hasBorder,
    {
      size: props.icon?.props.size,
      color: props.icon?.props.color,
    },
  );

  const icon = props.icon
    ? cloneElement(props.icon, {
        style: styles.icon,
      })
    : undefined;

  const iconOnly = !!icon && !children;

  return (
    <Pressable
      {...props}
      disabled={disabled}
      style={(state) => [
        styles.container,
        iconOnly && styles.iconContainer,
        state.pressed && styles.pressedStyle,
        typeof style === 'function' ? style(state) : style,
      ]}
      role="button"
    >
      {icon}

      {children && (
        <Typography
          variant="button"
          style={[styles.typography, props.typographyStyle]}
          selectable={false}
        >
          {children}
        </Typography>
      )}
    </Pressable>
  );
}

const BORDER_WIDTH = 1;
function buildStyles(
  theme: Theme,
  scheme: ButtonSchemes | 'disabled',
  rounded: boolean | undefined,
  hasBorder: boolean | undefined,
  iconStyle: {
    size?: number;
    color?: ColorValue;
  },
) {
  const componentStyle = theme.components.Button;
  const componentScheme = componentStyle.scheme[scheme];

  const useBorder =
    hasBorder !== undefined ? hasBorder : scheme === 'secondary' ? true : false;

  const padding = theme.spacings.x2;

  return StyleSheet.create({
    container: {
      padding,

      flexDirection: 'row',
      alignItems: 'center',
      gap: theme.spacings.x2,

      backgroundColor: componentScheme.backgroundColor,
      borderRadius: rounded ? theme.rounded.full : theme.rounded.base,
      ...(useBorder
        ? {
            borderWidth: BORDER_WIDTH,
            borderColor: componentScheme.borderColor,
          }
        : {}),
    },
    iconContainer: {
      width:
        (iconStyle.size ?? componentStyle.icon.size) +
        2 * padding +
        (useBorder ? 2 * BORDER_WIDTH : 0),
      aspectRatio: 1,

      justifyContent: 'center',
    },
    hoveredStyle: {
      backgroundColor: componentScheme.hovered.backgroundColor,
    },
    pressedStyle: {
      backgroundColor: componentScheme.pressed.backgroundColor,
    },
    typography: {
      color: componentStyle.scheme[scheme].color,
      userSelect: 'none',
    },
    icon: {
      fontSize: iconStyle.size ?? componentStyle.icon.size,
      color: iconStyle.color ?? componentStyle.scheme[scheme].color,
    },
  });
}

export function Demo() {
  const [themeMode, setThemeMode] = useThemeMode();

  return (
    /* eslint-disable react-native/no-inline-styles */
    <View
      style={[
        StyleSheet.absoluteFill,
        { gap: 16, alignItems: 'flex-start', padding: 10 },
      ]}
    >
      <Button>Default</Button>
      <Button scheme="primary">Primary</Button>
      <Button scheme="secondary">Secondary</Button>
      <Button scheme="tertiary">Tertiary</Button>

      <Button
        scheme="danger"
        icon={<Icon name="warning" />}
      >
        Danger
      </Button>
      <Button disabled>Disabled</Button>
      <Button icon={<Icon name="home-outline" />} />
      <Button icon={<Icon name="add" />}>Button with Icon</Button>

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
