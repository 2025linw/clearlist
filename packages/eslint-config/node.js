// @ts-nocheck

import prettier from 'eslint-plugin-prettier/recommended';
import tseslint from 'typescript-eslint';

export default [
  {
    ignores: ['**/dist/**', '**/node_modules/**'],
  },
  ...tseslint.configs.recommended,
  prettier,
  {
    rules: {
      'prettier/prettier': 'warn',
    },
  },
];
