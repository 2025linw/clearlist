export enum Cmp {
  Equal = '=',
  NotEqual = '!=',
  Less = '<',
  LessEq = '<=',
  Greater = '>',
  GreaterEq = '>=',
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
