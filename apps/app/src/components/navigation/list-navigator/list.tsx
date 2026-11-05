import { StyleSheet, View } from 'react-native';

import Button from '@components/primitive/button';
import HorizontalDivider from '@components/primitive/horizontal-divider';
import Icon from '@components/primitive/icon';

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
          iconName="file-tray"
          iconColor="skyblue"
          expanded={expanded}
        >
          Inbox
        </NavButton>

        <HorizontalDivider />

        <NavButton
          href="/lists/today"
          iconName="today"
          iconColor="#EAB308"
          expanded={expanded}
        >
          Today
        </NavButton>
        <NavButton
          href="/lists/upcoming"
          iconName="calendar"
          iconColor="red"
          expanded={expanded}
        >
          Upcoming
        </NavButton>
        <NavButton
          href="/lists/deadline"
          iconName="flag"
          iconColor="red"
          expanded={expanded}
        >
          Deadline
        </NavButton>

        <HorizontalDivider />

        <NavButton
          href="/lists/logbook"
          iconName="checkmark-circle"
          iconColor="green"
          expanded={expanded}
        >
          Logbook
        </NavButton>
        <NavButton
          href="/lists/trash"
          iconName="trash-bin"
          iconColor="gray"
          expanded={expanded}
        >
          Trash
        </NavButton>

        <HorizontalDivider />

        <Button
          scheme="tertiary"
          leftIcon={
            <Icon
              name="search"
              color="white"
            />
          }
        >
          {expanded && 'Search'}
        </Button>

        <HorizontalDivider />
      </View>

      <View>
        <NavButton
          href="/settings"
          iconName="settings"
          iconColor="gray"
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
