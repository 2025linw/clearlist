import { type ColorVariantName, type ThemeMode } from '../types';
import { colorThemes } from './themes';

export { colorThemes } from './themes';
export * as colors from './colors';

export function getColorThemes(mode: ThemeMode) {
  const out = Object.entries(colorThemes).map(([name, colors]) => {
    const color = colors[mode]['primary']['primary-9'];

    return [name as ColorVariantName, color] as const;
  });

  return out;
}
