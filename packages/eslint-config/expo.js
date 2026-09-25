// @ts-nocheck

import expoConfig from 'eslint-config-expo/flat.js';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import reactNative from 'eslint-plugin-react-native';
import tanstackQuery from '@tanstack/eslint-plugin-query'

import testingLibrary from 'eslint-plugin-testing-library';

import noSingleStyleArray from './rules/no-single-style-array.mjs';

const testFiles = [
  '**/__tests__/**/*.{js,jsx,ts,tsx}',
  '**/*.{spec,test}.{js,jsx,ts,tsx}',
];

export default [
  {
    ignores: ['**/dist/**', '**/node_modules/**'],
  },
  ...expoConfig,
  ...tanstackQuery.configs['flat/recommended'],
  {
    plugins: {
      'react-native': reactNative,
      'local': {
        rules: {
          'no-single-style-array': noSingleStyleArray,
        },
      },
    },
    rules: {
      'react-native/no-inline-styles': 'warn',
      'local/no-single-style-array': 'warn',
      '@typescript-eslint/consistent-type-imports': [
        'error',
        {
          prefer: 'type-imports',
          fixStyle: 'inline-type-imports',
        },
      ],
    },
  },
  {
    files: testFiles,
    ...testingLibrary.configs['flat/react'],
    settings: {
      'testing-library/utils-module': '@testing-library/react-native',
    },
  },
  eslintPluginPrettierRecommended,
  {
    rules: {
      'prettier/prettier': 'warn',
    },
  },
];
