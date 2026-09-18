import type { ColorScale } from '@/context/theme/colors/types';

export type { ColorVariant } from './types';

export * as main from './variant/default';

const red = {
  'red-1': '#fffcfc',
  'red-2': '#fff7f7',
  'red-3': '#feebec',
  'red-4': '#ffdbdc',
  'red-5': '#ffcdce',
  'red-6': '#fdbdbe',
  'red-7': '#f4a9aa',
  'red-8': '#eb8e90',
  'red-9': '#e5484d',
  'red-10': '#dc3e42',
  'red-11': '#ce2c31',
  'red-12': '#641723',
};

const orange = {
  'orange-1': '#fefcfb',
  'orange-2': '#fff7ed',
  'orange-3': '#ffefd6',
  'orange-4': '#ffdfb5',
  'orange-5': '#ffd19a',
  'orange-6': '#ffc182',
  'orange-7': '#f5ae73',
  'orange-8': '#ec9455',
  'orange-9': '#f76b15',
  'orange-10': '#ef5f00',
  'orange-11': '#cc4e00',
  'orange-12': '#582d1d',
};

const yellow = {
  'yellow-1': '#fdfdf9',
  'yellow-2': '#fefce9',
  'yellow-3': '#fffab8',
  'yellow-4': '#fff394',
  'yellow-5': '#ffe770',
  'yellow-6': '#f3d768',
  'yellow-7': '#e4c767',
  'yellow-8': '#d5ae39',
  'yellow-9': '#ffe629',
  'yellow-10': '#ffdc00',
  'yellow-11': '#9e6c00',
  'yellow-12': '#473b1f',
};

const green = {
  'green-1': '#fbfefc',
  'green-2': '#f4fbf6',
  'green-3': '#e6f6eb',
  'green-4': '#d6f1df',
  'green-5': '#c4e8d1',
  'green-6': '#adddc0',
  'green-7': '#8eceaa',
  'green-8': '#5bb98b',
  'green-9': '#30a46c',
  'green-10': '#2b9a66',
  'green-11': '#218358',
  'green-12': '#193b2d',
};

const blue = {
  'blue-1': '#fbfdff',
  'blue-2': '#f4faff',
  'blue-3': '#e6f4fe',
  'blue-4': '#d5efff',
  'blue-5': '#c2e5ff',
  'blue-6': '#acd8fc',
  'blue-7': '#8ec8f6',
  'blue-8': '#5eb1ef',
  'blue-9': '#0090ff',
  'blue-10': '#0588f0',
  'blue-11': '#0d74ce',
  'blue-12': '#113264',
};

const purple = {
  'purple-1': '#fefcfe',
  'purple-2': '#fbf7fe',
  'purple-3': '#f7edfe',
  'purple-4': '#f2e2fc',
  'purple-5': '#ead5f9',
  'purple-6': '#e0c4f4',
  'purple-7': '#d1afec',
  'purple-8': '#be93e4',
  'purple-9': '#8e4ec6',
  'purple-10': '#8347b9',
  'purple-11': '#8145b5',
  'purple-12': '#402060',
};

type Colors = 'red' | 'orange' | 'yellow' | 'green' | 'blue' | 'purple';
export const colors: {
  [K in Colors]: ColorScale<K>;
} = {
  red,
  orange,
  yellow,
  green,
  blue,
  purple,
};
