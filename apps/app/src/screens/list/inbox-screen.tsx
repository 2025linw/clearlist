import { Category } from '@/services/types';

import ListScreen from '@/screens/list/base-list-screen';

export default function InboxScreen() {
  return (
    <ListScreen
      headerText="Inbox"
      category={Category.Inbox}
    />
  );
}
