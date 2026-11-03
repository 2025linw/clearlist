import { useRoute, useRouter } from 'expo-router';
import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import Layout from '@components/layout';
import Button from '@components/primitive/button';
import Typography from '@components/primitive/typography';

export default function NotFoundScreen() {
  const route = useRoute();
  const router = useRouter();
  const theme = useTheme();

  const styles = buildStyles(theme);
  console.log(route);

  return (
    <Layout
      showBackButton
      style={styles.container}
    >
      <Typography>This page does not exist</Typography>

      <Button onPress={() => router.navigate('/')}>Go back</Button>
    </Layout>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      alignItems: 'center',
      justifyContent: 'center',
      gap: theme.spacings.x8,
    },
  });
}
