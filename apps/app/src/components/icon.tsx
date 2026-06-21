import Ionicons from '@react-native-vector-icons/ionicons/static';
import { ComponentProps } from 'react';
import { StyleSheet } from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

type Props = ComponentProps<typeof Ionicons>;
export type IconName = Props['name'];
export type IconColor = Props['color'];

export default function Icon({ style, size, color, ...props }: Props) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return (
    <Ionicons
      {...props}
      size={size || theme.components.Icon.size}
      color={color || theme.components.Icon.color}
      style={[styles.container, style]}
    />
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      justifyContent: 'center',
      alignItems: 'center',
    },
  });
}
