import Icon from '@components/primitives/icon';
import ListScreen from '@screens/list-screen';

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
