import { Category } from '@/services/types';

import ListScreen from '@/screens/list/base-list-screen';

export default function TodayScreen() {
  return (
    <ListScreen
      listName="Today"
      category={Category.Today}
    />
  );
}
