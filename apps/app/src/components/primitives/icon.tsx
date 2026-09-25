import Ionicons from '@react-native-vector-icons/ionicons';
import { type ComponentProps } from 'react';
import { type StyleProp, StyleSheet, View, type ViewStyle } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

type IoniconType = ComponentProps<typeof Ionicons>;
export type IconName = IoniconType['name'];
export type IconColor = IoniconType['color'];

type IconVariant = keyof Theme['components']['Icon']['variants'];

export type IconProps = IoniconType & {
  variant?: IconVariant;
  containerStyle?: StyleProp<ViewStyle>;
};

export default function Icon({
  variant = 'default',
  size,
  color,
  style,
  containerStyle,
  role,
  testID,
  ...props
}: IconProps) {
  const { components } = useTheme();

  const componentStyle = components.Icon.variants[variant];

  const flattenedStyle = StyleSheet.flatten(style);
  const resolvedSize = size ?? flattenedStyle?.fontSize ?? componentStyle.size;

  const styles = buildStyles(resolvedSize);

  if (__DEV__ && size !== undefined && flattenedStyle?.fontSize !== undefined) {
    console.warn(
      'Icon: both size and style.fontSize were specified; size takes precedence.',
    );
  }

  return (
    <View
      style={[styles.container, containerStyle]}
      role={role}
      testID={testID}
    >
      <Ionicons
        {...props}
        size={resolvedSize}
        color={color ?? componentStyle.color}
        style={[style, { fontSize: resolvedSize }]}
      />
    </View>
  );
}

function buildStyles(inputSize?: number) {
  return StyleSheet.create({
    container: {
      width: inputSize,
      height: inputSize,

      alignItems: 'center',
      justifyContent: 'center',
    },
  });
}
