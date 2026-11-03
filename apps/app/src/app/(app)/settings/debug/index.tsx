import { type Href, useRouter } from 'expo-router';

import Layout from '@components/layout';
import Button from '@components/primitive/button';
import HorizontalDivider from '@components/primitive/horizontal-divider';

export default function DebugPage() {
  const router = useRouter();

  return (
    <Layout
      headerText={'Debug'}
      showBackButton
    >
      <Button onPress={() => router.navigate('/settings/debug/spacing-debug')}>
        Spacing Debug
      </Button>

      <HorizontalDivider />

      <Button
        onPress={() => router.navigate('/settings/debug/typography-debug')}
      >
        Typography Debug
      </Button>
      <Button onPress={() => router.navigate('/settings/debug/button-debug')}>
        Button Debug
      </Button>
      <Button onPress={() => router.navigate('/settings/debug/checkbox-debug')}>
        Checkbox Debug
      </Button>
      <Button
        onPress={() => router.navigate('/settings/debug/text-input-debug')}
      >
        Text Input Debug
      </Button>
      <Button
        onPress={() =>
          router.navigate('/settings/debug/editable-typography-debug')
        }
      >
        Editable Typography Debug
      </Button>

      <HorizontalDivider />

      <Button onPress={() => router.navigate('/settings/debug/date-debug')}>
        Date Debug
      </Button>
      <Button onPress={() => router.navigate('/settings/debug/toast-debug')}>
        Toast Debug
      </Button>

      <HorizontalDivider />

      <Button
        onPress={() => {
          router.navigate('/invalid-route' as Href);
        }}
      >
        Invalid Route Debug
      </Button>
    </Layout>
  );
}
