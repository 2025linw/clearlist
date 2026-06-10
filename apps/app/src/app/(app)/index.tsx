import Ionicons from '@react-native-vector-icons/ionicons/static';
import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

// import { Tag } from '@clearlist/types';

// import { useNotificationContext } from '@/context/error';
import { useTheme } from '@/context/theme';

import Layout from '@/components/layout';
import Button from '@/components/primitives/button';
import HorizontalDivider from '@/components/primitives/horizontal-divider';
import Typography from '@/components/primitives/typography';

export default function Index() {
  const router = useRouter();
  const theme = useTheme();
  // const { showError } = useNotificationContext();

  // const [tags, setTags] = useState<Tag[] | null>(null);

  // useEffect(() => {
  //   getTags()
  //     .then((tags) => {
  //       setTags(tags);
  //     })
  //     .catch(() => {
  //       // showError('Unable to get tags');
  //     });
  // }, []);

  return (
    <Layout>
      <View style={styles.navContainer}>
        <Button
          text="Inbox"
          leftIcon={
            <Ionicons
              name="file-tray"
              size={18}
              color="skyblue"
            />
          }
          onPress={() => router.navigate('/lists/inbox')}
        />

        <HorizontalDivider />

        <Button
          text="Today"
          leftIcon={
            <Ionicons
              name="sunny-sharp"
              size={18}
              color="yellow"
            />
          }
          onPress={() => router.navigate('/lists/today')}
        />
        <Button
          text="Upcoming"
          leftIcon={
            <Ionicons
              name="calendar"
              size={18}
              color="red"
            />
          }
          onPress={() => router.navigate('/lists/upcoming')}
        />
        <Button
          text="Deadline"
          leftIcon={
            <Ionicons
              name="flag"
              size={18}
              color="red"
            />
          }
          onPress={() => router.navigate('/lists/deadline')}
        />

        <HorizontalDivider />

        <Button
          text="Logbook"
          leftIcon={
            <Ionicons
              name="checkmark-circle"
              size={18}
              color="green"
            />
          }
          onPress={() => router.navigate('/lists/logbook')}
        />
        <Button
          text="Trash"
          leftIcon={
            <Ionicons
              name="trash-bin"
              size={18}
              color="gray"
            />
          }
          onPress={() => router.navigate('/lists/trash')}
        />

        <HorizontalDivider />

        <Typography style={{ color: theme.palette.text }}>Tags</Typography>

        {/* <FlatList
          data={tags}
          keyExtractor={(tag) => tag.id}
          renderItem={({ item }) => (
            <View>
              <Typography style={{ color: theme.palette.text }}>
                {item.label}
              </Typography>
            </View>
          )}
        /> */}

        <HorizontalDivider />

        <Button
          text="Settings"
          leftIcon={
            <Ionicons
              name="settings"
              size={18}
              color={'gray'}
            />
          }
          onPress={() => router.navigate('/settings')}
        />
      </View>
    </Layout>
  );
}

const styles = StyleSheet.create({
  navContainer: {
    padding: 10,
  },
});
