// @ts-nocheck

import prettierRecommended from 'eslint-plugin-prettier/recommended';

export default [
  {
    ignores: ['**/dist/**', '**/node_modules/**'],
  },
  {
    files: ['**/*.js', '**/*.mjs', '**/*.cjs'],
  },
  prettierRecommended,
  {
    rules: {
      'prettier/prettier': 'warn',
    },
  },
];
