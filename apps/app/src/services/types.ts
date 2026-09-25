import type dayjs from '@lib/datetime';

export enum Cmp {
  Equal = 'eq',
  NotEqual = 'ne',
  Less = 'lt',
  LessEq = 'lte',
  Greater = 'gt',
  GreaterEq = 'gte',
}

export type DateQuery =
  | { type: 'ex'; state: boolean }
  | { type: 'eq'; date: dayjs.Dayjs }
  | {
      type: 'cmp';
      cmp: Cmp;
      date: dayjs.Dayjs;
    };

export type TaskQuery = {
  startDate?: DateQuery;
  deadline?: DateQuery;

  completed?: boolean;
  deleted?: boolean;

  sortBy?: string;
  sortOrder?: string;
};

export type Category =
  'inbox' | 'today' | 'upcoming' | 'deadline' | 'logged' | 'trash';
