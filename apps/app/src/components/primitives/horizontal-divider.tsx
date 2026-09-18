import { StyleSheet, View } from 'react-native';

import { useTheme } from '@/context/theme';

export default function HorizontalDivider() {
  const theme = useTheme();

  return (
    <View style={styles.container}>
      <View style={[styles.line, { backgroundColor: theme.palette.border }]} />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    // TODO: use theme spacing
    paddingHorizontal: 3,
    paddingVertical: 5,
  },
  line: {
    height: StyleSheet.hairlineWidth,
  },
});
