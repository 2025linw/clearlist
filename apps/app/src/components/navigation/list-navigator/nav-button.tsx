import { type Href, usePathname, useRouter } from 'expo-router';
import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';

import Button from '@components/primitive/button';
import Icon, {
  type IconColor,
  type IconName,
} from '@components/primitive/icon';

type NavButtonProps = {
  children: string;
  href: Href;
  iconName?: IconName;
  iconColor?: IconColor;
  expanded?: boolean;
};

export default function NavButton({
  children,
  href,
  expanded = true,
  ...props
}: NavButtonProps) {
  const route = usePathname();
  const router = useRouter();
  const theme = useTheme();
  const { gtTablet } = useBreakpoints();

  const isActive = route === href;
  const styles = buildStyles(theme);

  return (
    <Button
      scheme={gtTablet && isActive ? 'primary' : 'tertiary'}
      leftIcon={
        <Icon
          name={props.iconName || 'list-outline'}
          color={props.iconColor || 'gray'}
        />
      }
      onPress={() => router.navigate(href)}
      style={styles.container}
    >
      {expanded ? children : undefined}
    </Button>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      borderRadius: 0,
      borderTopLeftRadius: theme.rounded.base,
      borderBottomLeftRadius: theme.rounded.base,
    },
  });
}
