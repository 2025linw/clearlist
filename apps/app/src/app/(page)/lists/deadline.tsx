import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list-screen';

export default function DeadlinePage() {
  return (
    <ListScreen
      listName="Deadline"
      category={Category.Deadline}
      listIcon={
        <Icon
          name="flag"
          color="red"
        />
      }
    />
  );
}
