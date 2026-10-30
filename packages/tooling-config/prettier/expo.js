import sortImportsPlugin from '@trivago/prettier-plugin-sort-imports';

import base from './base.js';

const config = {
  ...base,

  jsxSingleQuote: false,
  singleAttributePerLine: true,

  plugins: [sortImportsPlugin],

  importOrder: [
    '<BUILTIN_MODULES>',
    '<THIRD_PARTY_MODULES>',
    '^(react($|/|-)|@react|expo($|/|-)|@expo)',
    '^@clearlist/types(/|$)',
    '^@(services|hooks|contexts|lib)(/|$)',
    '^(@components|#)',
    '^@testUtils',
    '^[../]',
    '^[./]',
  ],
  importOrderSeparation: true,
  importOrderSortSpecifiers: true,
};

export default config;
