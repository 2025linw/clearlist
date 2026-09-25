import { type Href, useRouter } from 'expo-router';

import { useBreakpoints } from '@contexts/theme/useBreakpoints';

import Button from '@components/primitives/button';
import Icon, {
  type IconColor,
  type IconName,
} from '@components/primitives/icon';

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
  const router = useRouter();
  const { gtTablet } = useBreakpoints();

  return (
    <Button
      scheme="tertiary"
      icon={
        <Icon
          name={props.iconName || 'list-outline'}
          color={props.iconColor || 'gray'}
        />
      }
      // onPress={() => (gtTablet ? router.replace(href) : router.push(href))}
      onPress={() => (gtTablet ? router.replace(href) : router.push(href))}
    >
      {expanded ? children : undefined}
    </Button>
  );
}
