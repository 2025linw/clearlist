import { Platform } from 'react-native';

const base = 4;
export const spacings = {
  x1: base * 1,
  x2: base * 2,
  x3: base * 3,
  x4: base * 4,
  x5: base * 5,
  x6: base * 6,
  x8: base * 8,
  x10: base * 10,
  x12: base * 12,
  x16: base * 16,
} as const;

export const rounded = {
  sm: 3,
  base: 6,
  lg: 10,
  full: 9999,
} as const;

export const typographyVariants = {
  h1: {
    fontFamily: 'Inter-Black',
    fontSize: 24,
  },
  h2: {
    fontFamily: 'Inter-Bold',
    fontSize: 20,
  },
  h3: {
    fontFamily: 'Inter-Bold',
    fontSize: 18,
  },
  h4: {
    fontFamily: 'Inter-Regular',
    fontSize: 13,
    textTransform: 'uppercase',
  },
  text: {
    fontFamily: 'Inter-Regular',
    fontSize: 16,
  },
  button: {
    fontFamily: 'Inter-Bold',
    fontSize: 18,
  },
} as const;

export const zHeight = {
  base: 0,

  content: 1,
  floating: 10,

  overlay: 100,
  modal: 1000,

  tooltip: 1200,

  max: 9999,
} as const;

export const shadows = {
  low: {
    ...Platform.select({
      ios: {
        shadowColor: '#000',
        shadowOffset: { width: 0, height: 2 },
        shadowOpacity: 0.15,
        shadowRadius: 1.2,
      },
      android: { elevation: 3 },
      web: {
        boxShadow: '0px 2px 1.2px rgba(0, 0, 0, 0.15)',
      },
    }),
  },
  base: {
    ...Platform.select({
      ios: {
        shadowColor: '#000',
        shadowOffset: { width: 0, height: 3 },
        shadowOpacity: 0.23,
        shadowRadius: 3.85,
      },
      android: { elevation: 6 },
      web: {
        boxShadow: '0px 3px 3.85px rgba(0, 0, 0, 0.23)',
      },
    }),
  },
  high: {
    ...Platform.select({
      ios: {
        shadowColor: '#000',
        shadowOffset: { width: 0, height: 5 },
        shadowOpacity: 0.38,
        shadowRadius: 6.37,
      },
      android: { elevation: 10 },
      web: {
        boxShadow: '0px 5px 6.37px rgba(0, 0, 0, 0.38)',
      },
    }),
  },
} as const;
