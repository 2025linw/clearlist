import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list-screen';

export default function UpcomingPage() {
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
