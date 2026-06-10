import { Tag, Task } from '@clearlist/types';

import { Response } from '@/lib/api-client';

export type TaskResponse = Response & {
  data: {
    count: number;
    tasks: Task[];
  };
};

export type TagResponse = Response & {
  data: {
    count: number;
    tags: Tag[];
  };
};
