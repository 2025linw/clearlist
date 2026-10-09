import { StyleSheet, View } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

export default function HorizontalDivider() {
  const theme = useTheme();
  const styles = buildStyles(theme);

  return (
    <View style={styles.container}>
      <View style={[styles.line, { backgroundColor: theme.palette.border }]} />
    </View>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      paddingHorizontal: theme.spacings.x2,
      paddingVertical: theme.spacings.x3,
    },
    line: {
      height: theme.spacings.thin,
    },
  });
}
