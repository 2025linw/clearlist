import Icon from '@components/primitive/icon';

import ListScreen from '@screens/list-screen';

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
