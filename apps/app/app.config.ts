import { ExpoConfig } from 'expo/config';

export default (): ExpoConfig => ({
  name: 'Clear List',
  slug: 'clearlist',
  scheme: 'clearlist',
  version: '0.0.1',
  owner: '2025linw',

  userInterfaceStyle: 'automatic',
  orientation: 'portrait',
  icon: './assets/images/icon.png',
  ios: {
    bundleIdentifier: 'com.saphy.clearlist',
    supportsTablet: true,
    infoPlist: {
      ITSAppUsesNonExemptEncryption: false,
      NSAppTransportSecurity: {
        NSAllowsLocalNetworking: true,
      },
    },
  },
  android: {
    package: 'com.saphy.clearlist',
    adaptiveIcon: {
      backgroundColor: '#E6F4FE',
      foregroundImage: './assets/images/android-icon-foreground.png',
      backgroundImage: './assets/images/android-icon-background.png',
      monochromeImage: './assets/images/android-icon-monochrome.png',
    },
    predictiveBackGestureEnabled: false,
  },
  web: {
    output: 'static',
    favicon: './assets/images/favicon.png',
  },
  plugins: [
    'expo-router',
    [
      'expo-splash-screen',
      {
        image: './assets/images/splash-icon.png',
        imageWidth: 200,
        resizeMode: 'contain',
        backgroundColor: '#ffffff',
        dark: {
          backgroundColor: '#000000',
        },
      },
    ],
    'expo-secure-store',
    [
      'expo-font',
      {
        fonts: ['./assets/fonts/Inter-Regular.otf'],
        android: {
          fonts: [
            {
              fontFamily: 'Inter',
              fontDefinitions: [
                {
                  path: './assets/fonts/Inter-Italic.otf',
                  weight: 400,
                  style: 'italic',
                },
                {
                  path: './assets/fonts/Inter-Medium.otf',
                  weight: 500,
                },
                {
                  path: './assets/fonts/Inter-Bold.otf',
                  weight: 700,
                },
                {
                  path: './assets/fonts/Inter-BoldItalic.otf',
                  weight: 700,
                  style: 'italic',
                },
                {
                  path: './assets/fonts/Inter-Black.otf',
                  weight: 900,
                },
              ],
            },
          ],
        },
        ios: {
          fonts: [
            './assets/fonts/Inter-Italic.otf',
            './assets/fonts/Inter-Medium.otf',
            './assets/fonts/Inter-Bold.otf',
            './assets/fonts/Inter-BoldItalic.otf',
            './assets/fonts/Inter-Black.otf',
          ],
        },
      },
    ],
    '@react-native-vector-icons/ionicons',
    'expo-image',
    'expo-web-browser',
  ],
  experiments: {
    typedRoutes: true,
    reactCompiler: true,
  },
  extra: {
    router: {},
    eas: {
      projectId: '591f5217-4558-47a7-8507-3dceadd18a02',
    },
  },
});
