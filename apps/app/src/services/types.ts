import { tag, task } from '@clearlist/types';

export enum Cmp {
  Equal = '=',
  NotEqual = '!=',
  Less = '<',
  LessEq = 'lte',
  Greater = '>',
  GreaterEq = 'gte',
}

export type DateQuery =
  | { type: 'ex'; state: boolean }
  | { type: 'eq'; date: Date }
  | {
      type: 'cmp';
      date: Date;
      cmp: Cmp;
    };

export type TaskQuery = {
  startDate?: DateQuery;
  deadline?: DateQuery;

  completed?: boolean;
  deleted?: boolean;
};

export enum Category {
  Inbox,
  Today,
  Upcoming,
  Deadline,
  Logged,
  Trash,
}

type Response = {
  message?: string;
  data?: unknown;
};

export type TaskResponse = Response & {
  data: task.Task;
};

export type TaskQueryResponse = Response & {
  data: {
    count: number;
    tasks: task.Task[];
  };
};

export type TagResponse = Response & {
  data: tag.Tag;
};

export type TagQueryResponse = Response & {
  data: {
    count: number;
    tags: tag.Tag[];
  };
};
