import Ionicons from '@expo/vector-icons/Ionicons';
import { Pressable, StyleSheet, View } from 'react-native';

import { Task } from '@/types/resource';

// import { useTheme } from '@/context/theme';

import Typography from '@/components/primitives/typography';

type TaskItemProp = {
  task: Task;
};

export default function TaskItem(props: TaskItemProp) {
  // const theme = useTheme();

  return (
    <View style={styles.container}>
      <Pressable>
        <View style={styles.checkbox}>
          {props.task.completed_at ? (
            <Ionicons
              name="checkbox"
              size={24}
            />
          ) : (
            <Ionicons
              name="square-outline"
              size={24}
            />
          )}
        </View>
      </Pressable>

      <Typography
        variant="button"
        palette={!props.task.title ? 'subtle' : undefined}
      >
        {props.task.title || 'New Todo'}
      </Typography>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    width: '100%',

    margin: 10,

    flexDirection: 'row',

    alignItems: 'center',
    justifyContent: 'flex-start',
  },
  checkbox: {
    marginRight: 5,
  },
});
