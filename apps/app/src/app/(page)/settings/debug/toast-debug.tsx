import { useToast } from '@hooks/use-toast';

import Layout from '@components/layout';
import Button from '@components/primitives/button';

export default function NotificationDebug() {
  const toast = useToast();

  return (
    <Layout>
      <Button onPress={() => toast.message('This is a default toast')}>
        Default
      </Button>
      <Button onPress={() => toast.success('This is a success toast')}>
        Success
      </Button>
      <Button onPress={() => toast.warn('This is an warn toast')}>Warn</Button>
      <Button onPress={() => toast.error('This is an error toast')}>
        Error
      </Button>
    </Layout>
  );
}
