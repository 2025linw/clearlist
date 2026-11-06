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
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';

import {
  COLLAPSED_WIDTH,
  SIDEBAR_MAX_WIDTH,
  SIDEBAR_MIN_WIDTH,
} from './constants';
import List from './list';

type ListNavigatorDevice = 'mobile' | 'tablet' | 'desktop';

type ListNavigatorProps = {
  type?: ListNavigatorDevice;
  width?: number;
  onWidthChange?: (width: number) => void;
};

const HANDLE_WIDTH = 25;
const HANDLE_HEIGHT = 80;
const HANDLE_DISTANCE = 10;

export default function ListNavigator({
  type = 'mobile',
  width = 240,
  onWidthChange,
}: ListNavigatorProps) {
  const theme = useTheme();
  const { top, bottom } = useSafeAreaInsets();
  const { gtTablet } = useBreakpoints();

  const [expanded, setExpanded] = useState(true);
  const sidebarWidth = useSharedValue(width);
  const startWidth = useSharedValue(width);

  const styles = buildStyles(theme, type);

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
        SIDEBAR_MAX_WIDTH,
        Math.max(SIDEBAR_MIN_WIDTH, startWidth.value + e.translationX),
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
        <List expanded={expanded} />
      </View>
    );
  }
}

function buildStyles(theme: Theme, mode: ListNavigatorDevice) {
  return StyleSheet.create({
    container: {
      position: 'relative',
      zIndex: 1,

      height: '100%',

      backgroundColor: theme.palette.surface,

      ...(mode !== 'mobile' && theme.shadows.high),
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
