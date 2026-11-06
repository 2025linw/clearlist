import Icon from '@components/primitive/icon';

import ListScreen from '@screens/list-screen';

export default function InboxPage() {
  return (
    <ListScreen
      listName="Inbox"
      category={'inbox'}
      listIcon={
        <Icon
          name="file-tray"
          color="skyblue"
        />
      }
    />
  );
}
