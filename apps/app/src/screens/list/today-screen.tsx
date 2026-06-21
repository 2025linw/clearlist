import { Category } from '@/services/types';

import ListScreen from '@/screens/list/base-list-screen';

export default function TodayScreen() {
  return (
    <ListScreen
      headerText="Today"
      category={Category.Today}
    />
  );
}
