// @ts-nocheck

import prettierRecommended from 'eslint-plugin-prettier/recommended';

export default [
  {
    files: ['**/*.js', '**/*.mjs', '**/*.cjs'],
  },
  prettierRecommended,
  {
    ignores: ['**/dist/**', '**/node_modules/**'],
    rules: {
      'prettier/prettier': 'warn',
    },
  },
];
