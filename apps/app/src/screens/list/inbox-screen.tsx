import { FlatList, StyleSheet } from 'react-native';

import { useTasks } from '@/hooks/use-tasks';
import { categoryQueryMap } from '@/services/helpers';
import { Category } from '@/services/types';

import AddTaskModal from '@/components/add-task-modal';
import Layout from '@/components/layout';
import TaskItem from '@/components/task-item';

export default function InboxScreen() {
  const searchQuery = categoryQueryMap[Category.Inbox];
  const query = useTasks(searchQuery);

  return (
    <>
      <Layout
        headerText="Inbox"
        showBackButton
      >
        <FlatList
          data={query.data}
          keyExtractor={(task) => task.id}
          renderItem={({ item }) => <TaskItem task={item} />}
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
