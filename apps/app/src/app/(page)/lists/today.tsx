import Icon from '@components/primitives/icon';
import ListScreen from '@screens/list-screen';

export default function TodayPage() {
  return (
    <ListScreen
      listName="Today"
      category={'today'}
      listIcon={
        <Icon
          name="today"
          color="#EAB308"
        />
      }
    />
  );
}
