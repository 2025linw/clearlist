import { Redirect } from 'expo-router';

export default function WebIndex() {
  return <Redirect href="/lists/inbox" />;
}
