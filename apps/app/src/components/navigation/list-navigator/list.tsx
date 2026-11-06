import { StyleSheet, View } from 'react-native';

import HorizontalDivider from '@components/primitive/horizontal-divider';

import { COLLAPSED_WIDTH } from './constants';
import NavButton from './nav-button';

type ListProps = {
  expanded?: boolean;
};

export default function List({ expanded = true }: ListProps) {
  const styles = buildStyles(expanded);

  return (
    <View style={styles.list}>
      <View>
        <NavButton
          href="/lists/inbox"
          icon={{ name: 'file-tray', color: 'skyblue' }}
          expanded={expanded}
        >
          Inbox
        </NavButton>

        <HorizontalDivider />

        <NavButton
          href="/lists/today"
          icon={{ name: 'today', color: '#EAB308' }}
          expanded={expanded}
        >
          Today
        </NavButton>
        <NavButton
          href="/lists/upcoming"
          icon={{ name: 'calendar', color: 'red' }}
          expanded={expanded}
        >
          Upcoming
        </NavButton>
        <NavButton
          href="/lists/deadline"
          icon={{ name: 'flag', color: 'red' }}
          expanded={expanded}
        >
          Deadline
        </NavButton>

        <HorizontalDivider />

        <NavButton
          href="/lists/logbook"
          icon={{ name: 'checkmark-circle', color: 'green' }}
          expanded={expanded}
        >
          Logbook
        </NavButton>
        <NavButton
          href="/lists/trash"
          icon={{ name: 'trash-bin', color: 'gray' }}
          expanded={expanded}
        >
          Trash
        </NavButton>

        <HorizontalDivider />

        <NavButton
          icon={{ name: 'search', color: 'white' }}
          expanded={expanded}
        >
          Search
        </NavButton>

        <HorizontalDivider />
      </View>

      <View>
        <NavButton
          href="/settings"
          icon={{ name: 'settings', color: 'gray' }}
          expanded={expanded}
        >
          Settings
        </NavButton>
      </View>
    </View>
  );
}

function buildStyles(expanded: boolean) {
  return StyleSheet.create({
    list: {
      flex: 1,

      justifyContent: 'space-between',
      ...(!expanded && { alignItems: 'flex-start' }),
    },
    button: {
      height: COLLAPSED_WIDTH,
    },
  });
}
