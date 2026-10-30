import { type StyleProp, StyleSheet, View, type ViewStyle } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

type MarginStyles =
  | 'margin'
  | 'marginVertical'
  | 'marginHorizontal'
  | 'marginTop'
  | 'marginBottom'
  | 'marginLeft'
  | 'marginRight';

type HorizontalDividerProps = {
  style?: StyleProp<Pick<ViewStyle, MarginStyles>>;
};

export default function HorizontalDivider(props: HorizontalDividerProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return <View style={[styles.line, props.style]} />;
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    line: {
      marginHorizontal: theme.spacings.x1,
      marginVertical: theme.spacings.x2,

      height: theme.spacings.thin,

      backgroundColor: theme.palette.border,
    },
  });
}
