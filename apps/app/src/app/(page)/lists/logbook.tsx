import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list-screen';

export default function DeadlinePage() {
  return (
    <ListScreen
      listName="Logbook"
      category={Category.Logged}
      listIcon={
        <Icon
          name="checkmark-circle"
          color="green"
        />
      }
    />
  );
}
