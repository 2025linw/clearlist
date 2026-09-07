import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list/base-list-screen';

export default function DeadlineScreen() {
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
