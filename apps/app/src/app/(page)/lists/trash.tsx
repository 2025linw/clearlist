import Icon from '@components/primitives/icon';
import ListScreen from '@screens/list-screen';

export default function TrashPage() {
  return (
    <ListScreen
      listName="Trash"
      category={'trash'}
      listIcon={
        <Icon
          name="trash-bin"
          color="gray"
        />
      }
    />
  );
}
