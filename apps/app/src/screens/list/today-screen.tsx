import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list/base-list-screen';

export default function TodayScreen() {
  return (
    <ListScreen
      listName="Today"
      category={Category.Today}
      listIcon={
        <Icon
          name="today"
          color="#EAB308"
        />
      }
    />
  );
}
