import ListScreen from '#/list-screen';
import Icon from '@components/primitive/icon';

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
