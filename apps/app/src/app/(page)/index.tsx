import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import Icon from '@/components/icon';
// import { useNotificationContext } from '@/context/error';

import Layout from '@/components/layout';
import Button from '@/components/primitives/button';
import HorizontalDivider from '@/components/primitives/horizontal-divider';

export default function Index() {
  const router = useRouter();

  return (
    <Layout style={styles.container}>
      <View>
        <Button
          icon={
            <Icon
              name="file-tray"
              color="skyblue"
            />
          }
          onPress={() => router.navigate('/lists/inbox')}
        >
          Inbox
        </Button>

        <HorizontalDivider />

        <Button
          icon={
            <Icon
              name="today"
              color="#EAB308"
            />
          }
          onPress={() => router.navigate('/lists/today')}
        >
          Today
        </Button>
        <Button
          icon={
            <Icon
              name="calendar"
              color="red"
            />
          }
          onPress={() => router.navigate('/lists/upcoming')}
        >
          Upcoming
        </Button>
        <Button
          icon={
            <Icon
              name="flag"
              color="red"
            />
          }
          onPress={() => router.navigate('/lists/deadline')}
        >
          Deadline
        </Button>

        <HorizontalDivider />

        <Button
          icon={
            <Icon
              name="checkmark-circle"
              color="green"
            />
          }
          onPress={() => router.navigate('/lists/logbook')}
        >
          Logbook
        </Button>
        <Button
          icon={
            <Icon
              name="trash-bin"
              color="gray"
            />
          }
          onPress={() => router.navigate('/lists/trash')}
        >
          Trash
        </Button>

        <HorizontalDivider />
      </View>

      <Button
        icon={
          <Icon
            name="settings"
            color="gray"
          />
        }
        onPress={() => router.navigate('/settings')}
      >
        Settings
      </Button>
    </Layout>
  );
}

const styles = StyleSheet.create({
  container: {
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'space-between',
  },
  navigationContainer: {},
});
