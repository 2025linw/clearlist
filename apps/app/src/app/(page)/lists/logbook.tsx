import Icon from '@components/primitives/icon';
import ListScreen from '@screens/list-screen';

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
