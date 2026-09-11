import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list/base-list-screen';

export default function InboxScreen() {
  return (
    <ListScreen
      listName="Inbox"
      category={Category.Inbox}
      listIcon={
        <Icon
          name="file-tray"
          color="skyblue"
        />
      }
    />
  );
}
