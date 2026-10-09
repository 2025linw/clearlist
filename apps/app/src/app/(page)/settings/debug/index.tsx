import { useRouter } from 'expo-router';

import Layout from '@components/layout';
import Button from '@components/primitives/button';
import HorizontalDivider from '@components/primitives/horizontal-divider';

export default function DebugPage() {
  const router = useRouter();

  return (
    <Layout
      headerText={'Debug'}
      showBackButton={true}
    >
      <Button onPress={() => router.navigate('/settings/debug/spacing-debug')}>
        Debug (Spacing)
      </Button>

      <HorizontalDivider />

      <Button
        onPress={() => router.navigate('/settings/debug/typography-debug')}
      >
        Debug (Typography)
      </Button>
      <Button onPress={() => router.navigate('/settings/debug/button-debug')}>
        Debug (Button)
      </Button>
      <Button onPress={() => router.navigate('/settings/debug/checkbox-debug')}>
        Debug (Checkbox)
      </Button>
      <Button
        onPress={() => router.navigate('/settings/debug/text-input-debug')}
      >
        Debug (Text Input)
      </Button>
      <Button
        onPress={() =>
          router.navigate('/settings/debug/editable-typography-debug')
        }
      >
        Debug (Editable Typography)
      </Button>

      <HorizontalDivider />

      <Button onPress={() => router.navigate('/settings/debug/toast-debug')}>
        Debug (Toast)
      </Button>
      <Button onPress={() => router.navigate('/settings/debug/date-debug')}>
        Debug (Date)
      </Button>
    </Layout>
  );
}
