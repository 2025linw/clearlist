import { StyleSheet, Text } from 'react-native';

import { spacings } from '@/context/theme/spacing';

import Layout from '@/components/layout';

export default function SpacingDebug() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <Layout>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x1 }]}
      >
        x1
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x2 }]}
      >
        x2
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x3 }]}
      >
        x3
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x4 }]}
      >
        x4
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x5 }]}
      >
        x5
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x6 }]}
      >
        x6
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x8 }]}
      >
        x8
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x10 }]}
      >
        x10
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x12 }]}
      >
        x12
      </Text>
      <Text
        numberOfLines={1}
        style={[styles.box, { width: spacings.x16 }]}
      >
        x16
      </Text>
    </Layout>
    /* eslint-enable react-native/no-inline-styles */
  );
}

const styles = StyleSheet.create({
  box: {
    backgroundColor: 'red',
  },
});
