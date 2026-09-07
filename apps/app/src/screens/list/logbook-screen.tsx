import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list/base-list-screen';

export default function DeadlineScreen() {
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
