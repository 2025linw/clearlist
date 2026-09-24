import { useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
} from 'react-native-reanimated';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { scheduleOnRN } from 'react-native-worklets';

import { useTheme } from '@contexts/theme';
import { Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@contexts/theme/useBreakpoints';

import HorizontalDivider from '@components/primitives/horizontal-divider';

import Button from './nav-button';

type ListNavigatorMode = 'mobile' | 'tablet' | 'desktop';
type ListNavigatorProps = {
  mode?: ListNavigatorMode;
  width?: number;
  onWidthChange?: (width: number) => void;
};

const HANDLE_WIDTH = 25;
const HANDLE_HEIGHT = 80;
const HANDLE_DISTANCE = 10;

const MIN_WIDTH = 180;
const MAX_WIDTH = 400;
const COLLAPSED_WIDTH = 40;

export default function ListNavigator({
  mode = 'mobile',
  width = 240,
  onWidthChange,
}: ListNavigatorProps) {
  const { top, bottom } = useSafeAreaInsets();
  const { gtTablet } = useBreakpoints();

  const [expanded, setExpanded] = useState(true);
  const sidebarWidth = useSharedValue(width);
  const startWidth = useSharedValue(width);

  const theme = useTheme();
  const styles = buildStyles(theme);

  const tapGesture = Gesture.Tap()
    .runOnJS(true)
    .onEnd((_e, success) => {
      if (success) setExpanded((expanded) => !expanded);
    });
  const panGesture = Gesture.Pan()
    .enabled(expanded)
    .onBegin(() => {
      startWidth.value = sidebarWidth.value;
    })
    .onUpdate((e) => {
      sidebarWidth.value = Math.min(
        MAX_WIDTH,
        Math.max(MIN_WIDTH, startWidth.value + e.translationX),
      );
    })
    .onEnd(() => {
      if (onWidthChange) {
        scheduleOnRN(onWidthChange, sidebarWidth.value);
      }
    });
  const composedGestures = Gesture.Exclusive(panGesture, tapGesture);

  const sidebarAnimatedStyle = useAnimatedStyle(() => ({
    width: sidebarWidth.value,
  }));

  if (gtTablet) {
    return (
      <Animated.View
        style={[
          styles.container,
          { paddingTop: top, paddingBottom: bottom },
          expanded ? sidebarAnimatedStyle : styles.collapsed,
        ]}
      >
        <List expanded={expanded} />

        <GestureDetector gesture={composedGestures}>
          <Animated.View style={styles.resizeHandle} />
        </GestureDetector>
      </Animated.View>
    );
  } else {
    return (
      <View
        style={[styles.container, { paddingTop: top, paddingBottom: bottom }]}
      >
        <List />
      </View>
    );
  }
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      position: 'relative',
      zIndex: 1,

      height: '100%',

      backgroundColor: theme.palette.background,
    },
    collapsed: {
      maxWidth: COLLAPSED_WIDTH,
    },
    resizeHandle: {
      position: 'absolute',
      top: '50%',
      left: '100%',
      marginLeft: HANDLE_DISTANCE,

      width: HANDLE_WIDTH,
      height: HANDLE_HEIGHT,

      transform: [{ translateY: -(HANDLE_HEIGHT / 2) }],
      zIndex: 10,

      borderRadius: theme.rounded.full,
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
