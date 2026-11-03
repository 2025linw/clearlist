import { getDateToday } from '@lib/datetime';

import Layout from '@components/layout';
import Typography from '@components/primitive/typography';

export default function DateDebug() {
  return (
    <Layout
      headerText="Date Debug"
      showBackButton
      scrollable
    >
      <Typography>{`getDateToday: ${getDateToday()}`}</Typography>
    </Layout>
  );
}
