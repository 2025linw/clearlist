import { type Href, usePathname, useRouter } from 'expo-router';
import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';

import { COLLAPSED_WIDTH } from '@components/navigation/list-navigator/constants';
import Button from '@components/primitive/button';
import Icon, {
  type IconColor,
  type IconName,
} from '@components/primitive/icon';

type NavButtonProps = {
  children: string;
  href?: Href;
  icon?: {
    name: IconName;
    color?: IconColor;
  };
  expanded?: boolean;
};

export default function NavButton({
  children,
  href,
  icon = {
    name: 'list-outline',
  },
  expanded = true,
}: NavButtonProps) {
  const route = usePathname();
  const router = useRouter();
  const theme = useTheme();
  const { gtTablet } = useBreakpoints();

  const isActive = route === href;
  const styles = buildStyles(theme, expanded);

  return (
    <Button
      scheme={gtTablet && isActive ? 'primary' : 'tertiary'}
      leftIcon={
        icon && (
          <Icon
            name={icon.name}
            color={icon.color || 'gray'}
          />
        )
      }
      onPress={() => {
        if (href) router.navigate(href);
      }}
      style={styles.container}
    >
      {children}
    </Button>
  );
}

function buildStyles(theme: Theme, expanded: boolean) {
  return StyleSheet.create({
    container: {
      borderRadius: 0,
      borderTopLeftRadius: theme.rounded.base,
      borderBottomLeftRadius: theme.rounded.base,

      maxWidth: expanded ? 'auto' : COLLAPSED_WIDTH,
      overflow: 'hidden',
    },
  });
}
