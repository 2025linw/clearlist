const { defineConfig } = require('eslint/config');
const expoConfig = require('eslint-config-expo/flat');
const eslintPluginPrettierRecommended = require('eslint-plugin-prettier/recommended');
const react = require('eslint-plugin-react');
const reactNative = require('eslint-plugin-react-native');

const noSingleStyleArray = require('./eslint-rules/no-single-style-array');

module.exports = defineConfig([
  expoConfig,
  eslintPluginPrettierRecommended,
  {
    ignores: ['dist/*'],
    plugins: {
      'react': react,
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
]);
