import { defineConfig } from 'eslint/config';
import expoConfig from 'eslint-config-expo/flat.js';
import reactNative from 'eslint-plugin-react-native';
import tanstackQuery from '@tanstack/eslint-plugin-query';
import testingLibraryPlugin from 'eslint-plugin-testing-library';

import base from './base.js';
import prettier from './prettier.js';

const testFiles = [
  '**/__tests__/**/*.{js,jsx,ts,tsx}',
  '**/*.{spec,test}.{js,jsx,ts,tsx}',
];

export default defineConfig([
  {
    ignores: ['**/*.config.{ts,mts,js,mjs}'],
  },
  base,
  expoConfig,
  tanstackQuery.configs['flat/recommended'],
  {
    plugins: {
      'react-native': reactNative,
    },
    rules: {
      'react-native/no-inline-styles': 'warn',
      'react-native/no-single-element-style-arrays': 'warn',
    },
    settings: {
      'import/resolver': {
        typescript: {
          extensions: [
            '.ts',
            '.tsx',
            '.native.ts',
            '.native.tsx',
            '.web.ts',
            '.web.tsx',
          ],
        },
      },
    },
  },
  {
    files: testFiles,
    extends: [testingLibraryPlugin.configs['flat/react']],
    settings: {
      'testing-library/utils-module': '@testing-library/react-native',
    },
  },
  prettier, // make sure that prettier is on bottom
]);
