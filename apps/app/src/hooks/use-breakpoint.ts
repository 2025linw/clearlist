import { useMemo } from 'react';
import { useWindowDimensions } from 'react-native';

export type Breakpoint = 'gtMobile' | 'gtTablet' | 'gtDesktop';

export function useBreakpoints(): Record<Breakpoint, boolean> & {
  activeBreakpoint?: Breakpoint;
} {
  const { width } = useWindowDimensions();

  const gtMobile = width >= 480;
  const gtTablet = width >= 768;
  const gtDesktop = width >= 992;

  return useMemo(() => {
    let active: Breakpoint | undefined;
    if (gtDesktop) {
      active = 'gtDesktop';
    } else if (gtTablet) {
      active = 'gtTablet';
    } else if (gtMobile) {
      active = 'gtMobile';
    }

    return {
      activeBreakpoint: active,
      gtMobile,
      gtTablet,
      gtDesktop,
    };
  }, [gtMobile, gtTablet, gtDesktop]);
}
