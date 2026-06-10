import { Category, Cmp, TaskQuery } from './types';

export function toYYYYMMDD(date: Date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');

  return `${year}-${month}-${day}`;
}

export const categoryQueryMap: Record<Category, TaskQuery> = {
  [Category.Inbox]: {
    startDate: { type: 'ex', state: false },
  },
  [Category.Today]: {
    startDate: { type: 'eq', date: new Date() },
  },
  [Category.Upcoming]: {
    startDate: { type: 'cmp', date: new Date(), cmp: Cmp.Greater },
  },
  [Category.Deadline]: {
    deadline: { type: 'ex', state: true },
  },
  [Category.Logged]: {
    completed: true,
  },
  [Category.Trash]: {
    deleted: true,
  },
};

export function buildTaskQuery(query: TaskQuery): string {
  const params = new URLSearchParams();

  if (query.startDate !== undefined) {
    const type = query.startDate.type;
    if (type === 'ex') {
      params.set('start', String(query.startDate.state));
    } else if (type === 'eq') {
      params.set('start', toYYYYMMDD(query.startDate.date));
    } else {
      params.set(
        `start[${query.startDate.cmp}]`,
        toYYYYMMDD(query.startDate.date),
      );
    }
  }
  if (query.deadline !== undefined) {
    const type = query.deadline.type;
    if (type === 'ex') {
      params.set('start', String(query.deadline.state));
    } else if (type === 'eq') {
      params.set('start', toYYYYMMDD(query.deadline.date));
    } else {
      params.set(
        `start[${query.deadline.cmp}]`,
        toYYYYMMDD(query.deadline.date),
      );
    }
  }

  if (query.completed !== undefined) {
    params.set('complete', String(query.completed));
  }
  if (query.deleted !== undefined) {
    params.set('deleted', String(query.deleted));
  }

  return params.toString();
}
