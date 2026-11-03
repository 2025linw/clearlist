import ListScreen from '#/list-screen';
import Icon from '@components/primitive/icon';

export default function DeadlinePage() {
  return (
    <ListScreen
      listName="Logbook"
      category={'logged'}
      listIcon={
        <Icon
          name="checkmark-circle"
          color="green"
        />
      }
    />
  );
}
