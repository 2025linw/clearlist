import { Pressable, PressableProps, StyleSheet, View } from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Icon, { IconColor, IconName } from '@/components/icon';
import Typography from '@/components/primitives/typography';

type ButtonContent =
  | { text: string; iconName?: IconName }
  | { text?: string; iconName: IconName };

type ButtonSchemes = keyof Omit<
  Theme['components']['Button']['scheme'],
  'disabled'
>;

export type ButtonProps = PressableProps &
  ButtonContent & {
    iconSize?: number;
    iconColor?: IconColor;
    scheme?: ButtonSchemes;
  };

export default function Button({
  text,
  scheme = 'default',
  iconName,
  iconSize,
  iconColor,
  disabled,
  ...pressableProps
}: ButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(theme, disabled ? 'disabled' : scheme);

  const isOnlyIcon = iconName !== undefined && text === undefined;

  return (
    <Pressable
      {...pressableProps}
      disabled={disabled}
      style={({ pressed }) => [
        isOnlyIcon ? styles.iconOnlyContainer : styles.container,
        pressed && styles.pressedStyle,
        pressableProps.style,
      ]}
    >
      {iconName && (
        <Icon
          name={iconName}
          size={iconSize || theme.components.Button.icon.size}
          color={iconColor}
        />
      )}

      {text && (
        <View style={styles.typographyContainer}>
          <Typography
            variant="button"
            style={styles.typography}
          >
            {text}
          </Typography>
        </View>
      )}
    </Pressable>
  );
}

function buildStyles(theme: Theme, scheme: ButtonSchemes | 'disabled') {
  const componentStyle = theme.components.Button;

  return StyleSheet.create({
    iconOnlyContainer: {},
    container: {
      paddingHorizontal: theme.spacings.lg,

      flexDirection: 'row',
      alignItems: 'center',
      borderRadius: theme.rounded.base,
      gap: theme.spacings.lg,

      backgroundColor: componentStyle.scheme[scheme].backgroundColor,
      borderColor: componentStyle.scheme[scheme].borderColor,
    },
    pressedStyle: {},
    typographyContainer: {
      paddingVertical: theme.spacings.lg,
    },
    typography: {
      color: componentStyle.scheme[scheme].color,
    },
  });
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16 }}>
      <Button text="Default" />
      <Button
        text="Primary"
        scheme="primary"
      />
      <Button
        text="Danger  "
        scheme="danger"
      />
      <Button
        text="Disabled"
        disabled
      />
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
