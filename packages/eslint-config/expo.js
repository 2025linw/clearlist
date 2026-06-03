// @ts-nocheck

import expoConfig from 'eslint-config-expo/flat.js';
import react from 'eslint-plugin-react';
import reactNative from 'eslint-plugin-react-native';

import base from './base.js';
import noSingleStyleArray from './rules/no-single-style-array.mjs';

export default [
  ...expoConfig,
  ...base,
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
      'prettier/prettier': 'warn',
      'react-native/no-inline-styles': 'warn',
      'local/no-single-style-array': 'warn',
    },
  },
];
