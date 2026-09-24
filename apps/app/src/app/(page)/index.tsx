import { Redirect } from 'expo-router';

import { useBreakpoints } from '@contexts/theme/useBreakpoints';

// import { useNotificationContext } from '@contexts/error';

import ListNavigator from '@components/navigation/list-navigator';

export default function Index() {
  const { gtTablet } = useBreakpoints();

  if (gtTablet) {
    return <Redirect href="/lists/inbox" />;
  } else {
    return <ListNavigator />;
  }
}
