import { ReactNode } from 'react';
import { StyleProp, View, ViewStyle } from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

type BoxRadius = keyof Theme['rounded'];

type BoxProps = {
  children: ReactNode;
  radius?: BoxRadius;
  style?: StyleProp<ViewStyle>;
};

export default function Box({ children, radius = 'base', ...props }: BoxProps) {
  const theme = useTheme();

  const styles: ViewStyle = {
    borderRadius: theme.rounded[radius],
  };

  return <View style={[styles, props.style]}>{children}</View>;
}
