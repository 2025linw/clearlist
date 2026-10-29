import { defineConfig } from 'eslint/config';
import prettierRecommended from 'eslint-plugin-prettier/recommended';

export default defineConfig([
  prettierRecommended,
  {
    rules: {
      'prettier/prettier': 'warn',
    },
  },
]);
