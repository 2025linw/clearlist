import { useMemo } from 'react';
import { useWindowDimensions } from 'react-native';

export type Breakpoint = 'gtMobile' | 'gtTablet';

export function useBreakpoints(): Record<Breakpoint, boolean> & {
  activeBreakpoint?: Breakpoint;
} {
  const { width } = useWindowDimensions();

  const gtMobile = width >= 800;
  const gtTablet = width >= 1300;

  return useMemo(() => {
    let active: Breakpoint | undefined;
    if (gtTablet) {
      active = 'gtTablet';
    } else if (gtMobile) {
      active = 'gtMobile';
    }

    return {
      activeBreakpoint: active,
      gtMobile,
      gtTablet,
    };
  }, [gtMobile, gtTablet]);
}
