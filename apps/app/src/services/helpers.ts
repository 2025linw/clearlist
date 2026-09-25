import dayjs, { getDateToday } from '@lib/datetime';

import { Category, Cmp, TaskQuery } from './types';

export function toYYYYMMDD(input_date: dayjs.Dayjs) {
  return input_date.format('YYYY-MM-DD');
}

export function getCategoryQueryMap(): Record<Category, TaskQuery> {
  const today = getDateToday();
  const tomorrow = today.add(1, 'day');

  return {
    inbox: {
      startDate: { type: 'ex', state: false },
    },
    today: {
      startDate: {
        type: 'cmp',
        date: tomorrow,
        cmp: Cmp.Less,
      },
      sortBy: 'start',
    },
    upcoming: {
      startDate: {
        type: 'cmp',
        date: tomorrow,
        cmp: Cmp.GreaterEq,
      },
      sortBy: 'start',
    },
    deadline: {
      deadline: { type: 'ex', state: true },
      sortBy: 'deadline',
    },
    logged: {
      completed: true,
      sortBy: 'completed',
      sortOrder: 'desc',
    },
    trash: {
      deleted: true,
    },
  };
}

export function buildTaskQuery(query: TaskQuery): string {
  const params = new URLSearchParams();

  if (query.startDate !== undefined) {
    const type = query.startDate.type;
    if (type === 'ex') {
      params.set('start', String(query.startDate.state));
    } else if (type === 'eq') {
      params.set('start', query.startDate.date.toISOString());
    } else {
      params.set(
        `start[${query.startDate.cmp}]`,
        query.startDate.date.toISOString(),
      );
    }
  }
  if (query.deadline !== undefined) {
    const type = query.deadline.type;
    if (type === 'ex') {
      params.set('deadline', String(query.deadline.state));
    } else if (type === 'eq') {
      params.set('deadline', toYYYYMMDD(query.deadline.date));
    } else {
      params.set(
        `deadline[${query.deadline.cmp}]`,
        toYYYYMMDD(query.deadline.date),
      );
    }
  }

  if (query.completed !== undefined) {
    params.set('completed', String(query.completed));
  }
  if (query.deleted !== undefined) {
    params.set('deleted', String(query.deleted));
  }

  if (query.sortBy !== undefined) {
    params.set('sort', query.sortBy);
  }
  if (query.sortOrder !== undefined) {
    params.set('order', query.sortOrder);
  }

  return params.toString();
}
