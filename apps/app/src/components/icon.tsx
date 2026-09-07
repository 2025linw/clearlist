import Ionicons from '@react-native-vector-icons/ionicons';
import { ComponentProps } from 'react';
import { StyleSheet, View } from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

export type IconProps = ComponentProps<typeof Ionicons>;
export type IconName = IconProps['name'];
export type IconColor = IconProps['color'];

export default function Icon({
  name,
  size,
  color,
  style,
  ...props
}: IconProps) {
  const theme = useTheme();
  const styles = buildStyles(theme, size);

  return (
    <View style={styles.container}>
      <Ionicons
        name={name}
        size={size || theme.components.Icon.size}
        color={color || theme.components.Icon.color}
        style={style}
        {...props}
      />
    </View>
  );
}

function buildStyles(theme: Theme, size?: number) {
  return StyleSheet.create({
    container: {
      width: size || theme.components.Icon.size,
      height: size || theme.components.Icon.size,
    },
  });
}
