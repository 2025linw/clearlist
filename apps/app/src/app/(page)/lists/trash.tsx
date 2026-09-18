import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list-screen';

export default function TrashPage() {
  return (
    <ListScreen
      listName="Trash"
      category={Category.Trash}
      listIcon={
        <Icon
          name="trash-bin"
          color="gray"
        />
      }
    />
  );
}
