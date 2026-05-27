import { FlatList, StyleSheet, View } from 'react-native';

import { Task } from '@/types';

import AddTaskModal from '@/components/add-task-modal';
import Layout from '@/components/layout';
import Typography from '@/components/primitives/typography';
import TaskItem from '@/components/task-item';

export type Props = {
  listName: string;

  tasks?: Task[] | null;

  emptyText?: string;
};

export default function ListScreen(props: Props) {
  return (
    <>
      <Layout
        headerText={props.listName}
        showBackButton
      >
        <FlatList
          data={props.tasks}
          keyExtractor={(task) => task.id}
          renderItem={({ item }) => <TaskItem task={item} />}
          contentContainerStyle={styles.listContainer}
          ListEmptyComponent={
            <View style={styles.emptyComponent}>
              <Typography>{props.tasks === null ? 'Loading tasks...' : props.emptyText || 'No tasks'}</Typography>
            </View>
          }
        />
      </Layout>

      <AddTaskModal
        style={styles.modalContainer}
        buttonStyle={styles.modalButton}
      />
    </>
  );
}

const styles = StyleSheet.create({
  listContainer: {
    flexGrow: 1,
  },
  emptyComponent: {
    flex: 1,

    alignItems: 'center',
    justifyContent: 'center',
  },
  modalContainer: {},
  modalButton: {
    bottom: 25,
    right: 25,
  },
});
