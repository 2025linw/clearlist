import ListScreen from '#/list-screen';
import Icon from '@components/primitive/icon';

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
