import { defineConfig } from 'eslint/config';
import globals from 'globals';

import base from './base.js';
import prettier from './prettier.js';

export default defineConfig([
  base,
  {
    languageOptions: {
      globals: globals.node,
    },
  },
  prettier, // make sure that prettier is on bottom
]);
