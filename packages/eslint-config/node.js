// @ts-nocheck

import tseslint from 'typescript-eslint';

import base from './base.js';

export default [...tseslint.configs.recommended, ...base];
