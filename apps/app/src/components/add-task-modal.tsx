import Ionicons from '@expo/vector-icons/Ionicons';
import { useState } from 'react';
import { Keyboard, Modal, Pressable, StyleProp, StyleSheet, View, ViewStyle } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, { useAnimatedStyle, useSharedValue, withSpring } from 'react-native-reanimated';
import { runOnJS } from 'react-native-worklets';

import { useTheme } from '@/context/theme';
import { addTasks } from '@/services/api';

import Button from '@/components/primitives/button';
import TextInput from '@/components/primitives/text-input';

type Props = {
  style?: StyleProp<ViewStyle>;
  buttonStyle?: Pick<ViewStyle, 'top' | 'bottom' | 'left' | 'right'>;
};

export default function AddTaskModal(props: Props) {
  const theme = useTheme();

  const [isLoading, setLoading] = useState(false);
  const [showModal, setShowModal] = useState(false);

  const [title, setTitle] = useState('');
  const [notes, setNotes] = useState('');

  async function handleSubmit() {
    if (isLoading) return;
    setLoading(true);

    try {
      await addTasks({
        title,
        notes,
      });

      setShowModal(false);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  }

  return (
    <>
      <Modal
        visible={showModal}
        onRequestClose={() => setShowModal(false)} // this is only used for HW close or gesture closes
        transparent
      >
        <Pressable
          style={[styles.backdrop, props.style]}
          onPress={() => setShowModal(false)}
        >
          <Pressable
            style={[
              styles.container,
              {
                backgroundColor: theme.palette.surface,
              },
            ]}
            onPress={(e) => {
              e.stopPropagation();

              Keyboard.dismiss();
            }}
          >
            <View style={[styles.content, props.style]}>
              <TextInput
                placeholder="New Task"
                onChangeText={(title) => setTitle(title)}
              />

              <TextInput
                placeholder="Notes"
                multiline
                textAlignVertical="top"
                style={styles.noteBox}
                onChangeText={(notes) => setNotes(notes)}
              />

              <Button
                text="Submit"
                scheme="primary"
                onPress={() => handleSubmit()}
              />
            </View>
          </Pressable>
        </Pressable>
      </Modal>

      <ModalButton
        style={[styles.button, props.buttonStyle]}
        onPressOut={() => setShowModal(true)}
      />
    </>
  );
}

type ButtonProps = {
  style?: StyleProp<ViewStyle>;
  onPressIn?: () => void;
  onPressOut?: () => void;
};

function ModalButton(props: ButtonProps) {
  const theme = useTheme();

  const isPressed = useSharedValue(false);
  const offset = useSharedValue({ x: 0, y: 0 });

  const start = useSharedValue({ x: 0, y: 0 });
  const gesture = Gesture.Pan()
    .onBegin(() => {
      isPressed.value = true;

      if (props.onPressIn) runOnJS(props.onPressIn)();
    })
    .onUpdate((e) => {
      offset.value = {
        x: e.translationX + start.value.x,
        y: e.translationY + start.value.y,
      };
    })
    .onEnd(() => {
      offset.value = {
        x: withSpring(0),
        y: withSpring(0),
      };
    })
    .onFinalize(() => {
      if (props.onPressOut) runOnJS(props.onPressOut)();

      isPressed.value = false;
    });

  const animatedStyles = useAnimatedStyle(() => {
    return {
      transform: [
        { translateX: offset.value.x },
        { translateY: offset.value.y },
        { scale: withSpring(isPressed.value ? 1.2 : 1) },
      ],
    };
  });

  return (
    <GestureDetector gesture={gesture}>
      <Animated.View style={[animatedStyles, props.style]}>
        <Ionicons
          name="add-circle"
          size={64}
          color={theme.palette.primary}
        />
      </Animated.View>
    </GestureDetector>
  );
}

const styles = StyleSheet.create({
  backdrop: {
    flex: 1,

    justifyContent: 'center',
    alignItems: 'center',
  },
  container: {
    width: '100%',
    height: '30%',

    padding: 25,
  },
  content: {
    flex: 1,

    gap: 12,
  },
  noteBox: {
    padding: 0,
    flex: 1,
  },
  button: {
    position: 'absolute',
  },
});
