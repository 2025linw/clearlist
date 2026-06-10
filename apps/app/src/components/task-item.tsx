import Ionicons from '@react-native-vector-icons/ionicons/static';
import { Pressable, StyleSheet, View } from 'react-native';

import { Task } from '@clearlist/types';

import { API_URL } from '@/constants';

// import { useTheme } from '@/context/theme';

import Typography from '@/components/primitives/typography';

import { apiFetch } from '@/lib/api-client';

type TaskItemProp = {
  task: Task;
};

export default function TaskItem({ task, ...props }: TaskItemProp) {
  // const theme = useTheme();

  async function toggleComplete() {
    if (task.completedAt) {
      apiFetch(API_URL + `/api/tasks/${task.id}/reopen`, {
        method: 'POST',
      });
    } else {
      apiFetch(API_URL + `/api/tasks/${task.id}/complete`, {
        method: 'POST',
      });
    }
  }

  console.log(task);

  return (
    <View style={styles.container}>
      <Pressable onPress={() => toggleComplete()}>
        {task.completedAt ? (
          <Ionicons
            name="checkbox"
            size={24}
            style={styles.checkbox}
          />
        ) : (
          <Ionicons
            name="square-outline"
            size={24}
            style={styles.checkbox}
          />
        )}
      </Pressable>

      <Typography
        variant="button"
        palette={!task.title ? 'subtle' : 'text'}
      >
        {task.title || 'New Todo'}
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
