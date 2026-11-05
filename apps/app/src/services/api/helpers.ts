import type dayjs from '@lib/datetime';
import { getDateToday } from '@lib/datetime';

import { type Category, Cmp, type QueryParams, type TaskQuery } from './types';

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

export function buildTaskQuery(query: TaskQuery): QueryParams {
  const params: QueryParams = {};
  if (query.startDate !== undefined) {
    const type = query.startDate.type;
    if (type === 'ex') {
      params['start'] = String(query.startDate.state);
    } else if (type === 'eq') {
      params['start'] = query.startDate.date.toISOString();
    } else {
      params[`start[${query.startDate.cmp}]`] =
        query.startDate.date.toISOString();
    }
  }
  if (query.deadline !== undefined) {
    const type = query.deadline.type;
    if (type === 'ex') {
      params['deadline'] = String(query.deadline.state);
    } else if (type === 'eq') {
      params['deadline'] = toYYYYMMDD(query.deadline.date);
    } else {
      params[`deadline[${query.deadline.cmp}]`] = toYYYYMMDD(
        query.deadline.date,
      );
    }
  }

  if (query.completed !== undefined) {
    params['completed'] = String(query.completed);
  }
  if (query.deleted !== undefined) {
    params['deleted'] = String(query.deleted);
  }

  if (query.sortBy !== undefined) {
    params['sort'] = query.sortBy;
  }
  if (query.sortOrder !== undefined) {
    params['order'] = query.sortOrder;
  }

  return params;
}
