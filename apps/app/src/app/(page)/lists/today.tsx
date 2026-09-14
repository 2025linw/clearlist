import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list-screen';

export default function TodayPage() {
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
