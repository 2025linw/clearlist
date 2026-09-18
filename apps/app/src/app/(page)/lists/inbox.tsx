import { Category } from '@/services/types';

import Icon from '@/components/icon';
import ListScreen from '@/screens/list-screen';

export default function InboxPage() {
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
