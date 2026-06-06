import { StyleSheet, View } from 'react-native';

export default function HorizontalDivider() {
  return <View style={styles.line} />;
}

const styles = StyleSheet.create({
  line: {
    height: StyleSheet.hairlineWidth,
    width: '100%',

    marginVertical: 5,

    backgroundColor: 'black',
  },
});
