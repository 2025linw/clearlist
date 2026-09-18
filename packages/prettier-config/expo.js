// @ts-nocheck

import sortImportsPlugin from '@trivago/prettier-plugin-sort-imports';

import base from './base.js';

export default {
  ...base,

  jsxSingleQuote: false,
  singleAttributePerLine: true,

  plugins: [sortImportsPlugin],

  importOrder: [
    '<BUILTIN_MODULES>',
    '<THIRD_PARTY_MODULES>',
    '^(react(.*)|@react|expo|@expo)',
    '^@clearlist/(.*)',
    '^@/(types|constants)',
    '^@/(services|context|hooks)',
    '^(@/components|@/screens/(.*))',
    '^@/(.*)',
    '^[../]',
    '^[./]',
  ],
  importOrderSeparation: true,
  importOrderSortSpecifiers: true,
};
