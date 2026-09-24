import { getDateToday, getDateTomorrow } from '@lib/datetime';

import Layout from '@components/layout';
import Typography from '@components/primitives/typography';

export default function DateDebug() {
  return (
    <Layout>
      <Typography>{`getDateToday: ${getDateToday()}`}</Typography>
      <Typography>{`getDateTomorrow: ${getDateTomorrow()}`}</Typography>
    </Layout>
  );
}
