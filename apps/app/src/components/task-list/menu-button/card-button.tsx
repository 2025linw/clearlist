import { useEffect, useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withTiming,
} from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';

import { useTheme } from '@contexts/theme';
import { Theme } from '@contexts/theme/types';

import Button from '@components/primitives/button';
import Icon from '@components/primitives/icon';

import {
  ACTION_BUTTON_SIZE,
  ACTION_DISTANCE,
  ACTION_ICON_SIZE,
  BUTTON_SIZE,
  ICON_SIZE,
} from './constants';

type CardButtonProps = {
  deleted: boolean;
  onTrashTask?: () => void;
  onRestoreTask?: () => void;
};

export default function CardButton({
  deleted,
  onTrashTask,
  onRestoreTask,
}: CardButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  const [menuOpen, setMenuOpen] = useState(false);
  const progress = useSharedValue(0);

  useEffect(() => {
    progress.value = withTiming(menuOpen ? 1 : 0, {
      duration: 200,
    });
  }, [progress, menuOpen]);

  const tapGesture = Gesture.Tap().onEnd((_e, success) => {
    if (success) {
      scheduleOnRN(setMenuOpen, (open) => !open);
    }
  });

  const dragGesture = Gesture.Pan()
    .onStart(() => {
      scheduleOnRN(setMenuOpen, true);
    })
    .onEnd(() => {
      scheduleOnRN(setMenuOpen, false);
    });
  const composedGesture = Gesture.Race(tapGesture, dragGesture);

  const trashButtonStyle = useAnimatedStyle(() => ({
    opacity: progress.value,
    transform: [
      {
        translateX:
          -(BUTTON_SIZE / 2 + ACTION_DISTANCE + ACTION_BUTTON_SIZE / 2) *
          progress.value,
      },
      { scale: progress.value },
    ],
  }));

  return (
    <View>
      <GestureDetector gesture={composedGesture}>
        <Animated.View>
          <Button
            rounded
            icon={
              <Icon
                name={menuOpen ? 'close' : 'menu'}
                size={ICON_SIZE}
              />
            }
            style={styles.button}
          />
        </Animated.View>
      </GestureDetector>

      {menuOpen && (
        <Animated.View style={[styles.actionButtonContainer, trashButtonStyle]}>
          <Button
            rounded
            icon={
              <Icon
                name={deleted ? 'return-up-back' : 'trash'}
                size={ACTION_ICON_SIZE}
              />
            }
            style={styles.actionButton}

            onPress={deleted ? onRestoreTask : onTrashTask}
          />
        </Animated.View>
      )}
    </View>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    button: {
      width: BUTTON_SIZE,
    },
    actionButtonContainer: {
      position: 'absolute',
      top: (BUTTON_SIZE - ACTION_BUTTON_SIZE) / 2,
      left: (BUTTON_SIZE - ACTION_BUTTON_SIZE) / 2,
    },
    actionButton: {
      width: ACTION_BUTTON_SIZE,
    },
  });
}
