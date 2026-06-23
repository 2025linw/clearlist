import { Category } from '@/services/types';

import ListScreen from '@/screens/list/base-list-screen';

export default function UpcomingScreen() {
  return (
    <ListScreen
      listName="Upcoming"
      category={Category.Upcoming}
    />
  );
}
