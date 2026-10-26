import ListScreen from '#/list-screen';
import Icon from '@components/primitives/icon';

export default function DeadlinePage() {
  return (
    <ListScreen
      listName="Deadline"
      category={'deadline'}
      listIcon={
        <Icon
          name="flag"
          color="red"
        />
      }
    />
  );
}
