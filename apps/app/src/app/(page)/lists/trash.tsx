import ListScreen from '#/list-screen';
import Icon from '@components/primitive/icon';

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
