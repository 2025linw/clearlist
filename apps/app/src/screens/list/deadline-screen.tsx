import { Category } from '@/services/types';

import ListScreen from '@/screens/list/base-list-screen';

export default function DeadlineScreen() {
  return (
    <ListScreen
      headerText="Deadline"
      category={Category.Deadline}
    />
  );
}
