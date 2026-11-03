import ListScreen from '#/list-screen';
import Icon from '@components/primitive/icon';

export default function UpcomingPage() {
  return (
    <ListScreen
      listName="Upcoming"
      category={'upcoming'}
      listIcon={
        <Icon
          name="calendar"
          color="red"
        />
      }
    />
  );
}
