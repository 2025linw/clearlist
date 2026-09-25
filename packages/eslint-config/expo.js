// @ts-nocheck

import expoConfig from 'eslint-config-expo/flat.js';
import react from 'eslint-plugin-react';
import reactNative from 'eslint-plugin-react-native';
import testingLibrary from 'eslint-plugin-testing-library';
import tanstackQuery from '@tanstack/eslint-plugin-query'

import base from './base.js';
import noSingleStyleArray from './rules/no-single-style-array.mjs';

const testFiles = [
  '**/__tests__/**/*.{js,jsx,ts,tsx}',
  '**/*.{spec,test}.{js,jsx,ts,tsx}',
];

export default [
  ...expoConfig,
  ...base,
  ...tanstackQuery.configs['flat/recommended'],
  {
    plugins: {
      react,
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
    },
    settings: {
      'import/resolver': {
        typescript: {
          project: true,
        },
      },
    },
  },
  {
    ...testingLibrary.configs['flat/react'],
    files: testFiles,
  },
];
