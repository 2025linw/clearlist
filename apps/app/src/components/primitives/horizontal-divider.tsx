import { StyleSheet, View } from 'react-native';

import { useTheme } from '@/context/theme';

export default function HorizontalDivider() {
  const theme = useTheme();

  return (
    <View style={[styles.line, { backgroundColor: theme.palette.border }]} />
  );
}

const styles = StyleSheet.create({
  line: {
    height: StyleSheet.hairlineWidth,
    width: '100%',

    marginVertical: 5,
  },
});
