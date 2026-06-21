import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

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
          text="Inbox"
          iconName="file-tray"
          iconColor="skyblue"
          onPress={() => router.navigate('/lists/inbox')}
        />

        <HorizontalDivider />

        <Button
          text="Today"
          iconName="today"
          iconColor="#EAB308"
          onPress={() => router.navigate('/lists/today')}
        />
        <Button
          text="Upcoming"
          iconName="calendar"
          iconColor="red"
          onPress={() => router.navigate('/lists/upcoming')}
        />
        <Button
          text="Deadline"
          iconName="flag"
          iconColor="red"
          onPress={() => router.navigate('/lists/deadline')}
        />

        <HorizontalDivider />

        <Button
          text="Logbook"
          iconName="checkmark-circle"
          iconColor="green"
          onPress={() => router.navigate('/lists/logbook')}
        />
        <Button
          text="Trash"
          iconName="trash-bin"
          iconColor="gray"
          onPress={() => router.navigate('/lists/trash')}
        />

        <HorizontalDivider />
      </View>

      <Button
        text="Settings"
        iconName="settings"
        iconColor="gray"
        onPress={() => router.navigate('/settings')}
      />
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
