import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list/base-list-screen';

export default function UpcomingScreen() {
  return (
    <ListScreen
      listName="Upcoming"
      category={Category.Upcoming}
      listIcon={
        <Icon
          name="calendar"
          color="red"
        />
      }
    />
  );
}
