import { useRef, useState } from 'react';
import {
  GestureResponderEvent,
  Pressable,
  StyleProp,
  StyleSheet,
  View,
  ViewStyle,
} from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';
import { useBreakpoints } from '@/context/theme/useBreakpoints';

import HorizontalDivider from '@/components/primitives/horizontal-divider';

import Button from './nav-button';

type ListNavigatorMode = 'mobile' | 'tablet' | 'desktop';
type ListNavigatorProps = {
  mode?: ListNavigatorMode;
  width?: number;
  onWidthChange?: (width: number) => void;
  style?: StyleProp<ViewStyle>;
};

const HANDLE_DISTANCE = 10;

const MIN_WIDTH = 180;
const MAX_WIDTH = 400;
const COLLAPSED_WIDTH = 40;

export default function ListNavigator({
  mode = 'mobile',
  width = 240,
  onWidthChange,
  ...props
}: ListNavigatorProps) {
  const theme = useTheme();

  const { top, bottom } = useSafeAreaInsets();
  const { gtTablet } = useBreakpoints();

  const [expanded, setExpanded] = useState(true);
  const resizing = useRef(false);
  const startX = useRef(0);
  const startWidth = useRef(width);

  const styles = buildStyles(mode, expanded, width, theme);

  function handleTap() {
    setExpanded(!expanded);
  }

  function handlePointerDown() {
    resizing.current = true;
    startX.current = width;
    startWidth.current = width;
  }

  function handlePointerMove(e: GestureResponderEvent) {
    if (!resizing.current) {
      return;
    }

    const delta = e.nativeEvent.pageX - startX.current;

    const nextWidth = Math.min(
      MAX_WIDTH,
      Math.max(MIN_WIDTH, startWidth.current + delta),
    );

    if (onWidthChange) onWidthChange(nextWidth);
  }

  function handlePointerUp() {
    resizing.current = false;
  }

  if (gtTablet) {
    return (
      <View
        style={[styles.wrapper, { paddingTop: top, paddingBottom: bottom }]}
      >
        <View style={[styles.container, props.style]}>
          <List expanded={expanded} />
        </View>

        <Pressable
          style={styles.resizeHandle}
          onPress={handleTap}
          onPressIn={handlePointerDown}
          onPressMove={handlePointerMove}
          onPressOut={handlePointerUp}
        />
      </View>
    );
  } else {
    return (
      <View
        style={[styles.wrapper, { paddingTop: top, paddingBottom: bottom }]}
      >
        <List />
      </View>
    );
  }
}

function buildStyles(
  mode: ListNavigatorMode,
  expanded: boolean,
  width: number,
  theme: Theme,
) {
  return StyleSheet.create({
    wrapper: {
      position: 'relative',
      zIndex: 1,

      height: '100%',

      backgroundColor: theme.palette.background,
    },
    container: {
      flex: 1,

      width: expanded ? width : null,

      flexDirection: 'column',
      justifyContent: 'space-between',
    },
    selected: {
      backgroundColor: 'red',
    },
    resizeHandle: {
      position: 'absolute',
      top: '50%',
      left: (expanded ? width : COLLAPSED_WIDTH) + HANDLE_DISTANCE,
      zIndex: 10,

      width: 25,
      height: 80,
      transform: [{ translateY: -40 }],

      borderRadius: theme.rounded.full,
      borderWidth: 0,

      backgroundColor: 'gray',
    },
  });
}

type ListProps = {
  expanded?: boolean;
};

function List({ expanded = true }: ListProps) {
  return (
    <View style={[styles.list, !expanded && styles.collapsedList]}>
      <View>
        <Button
          href="/lists/inbox"
          iconName="file-tray"
          iconColor="skyblue"
          expanded={expanded}
        >
          Inbox
        </Button>

        <HorizontalDivider />

        <Button
          href="/lists/today"
          iconName="today"
          iconColor="#EAB308"
          expanded={expanded}
        >
          Today
        </Button>
        <Button
          href="/lists/upcoming"
          iconName="calendar"
          iconColor="red"
          expanded={expanded}
        >
          Upcoming
        </Button>
        <Button
          href="/lists/deadline"
          iconName="flag"
          iconColor="red"
          expanded={expanded}
        >
          Deadline
        </Button>

        <HorizontalDivider />

        <Button
          href="/lists/logbook"
          iconName="checkmark-circle"
          iconColor="green"
          expanded={expanded}
        >
          Logbook
        </Button>
        <Button
          href="/lists/trash"
          iconName="trash-bin"
          iconColor="gray"
          expanded={expanded}
        >
          Trash
        </Button>

        <HorizontalDivider />
      </View>

      <View>
        <Button
          href="/settings"
          iconName="settings"
          iconColor="gray"
          expanded={expanded}
        >
          Settings
        </Button>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  list: {
    flex: 1,

    justifyContent: 'space-between',
  },

  collapsedList: {
    alignSelf: 'flex-start',
  },
});
