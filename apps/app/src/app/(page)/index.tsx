import { Redirect } from 'expo-router';

import { useBreakpoints } from '@/context/theme/useBreakpoints';

// import { useNotificationContext } from '@/context/error';

import ListNavigator from '@/components/navigation/list-navigator';

export default function Index() {
  const { gtTablet } = useBreakpoints();

  if (gtTablet) {
    return <Redirect href="/(page)/lists/inbox" />;
  } else {
    return <ListNavigator />;
  }
}
