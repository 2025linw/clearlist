import { Category } from '@/services/types';

import ListScreen from '@/screens/list/base-list-screen';

export default function TrashScreen() {
  return (
    <ListScreen
      listName="Trash"
      category={Category.Trash}
    />
  );
}
