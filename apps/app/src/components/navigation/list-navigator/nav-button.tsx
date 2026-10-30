import { type Href, useRouter } from 'expo-router';

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
  const router = useRouter();

  return (
    <Button
      scheme="tertiary"
      icon={
        <Icon
          name={props.iconName || 'list-outline'}
          color={props.iconColor || 'gray'}
        />
      }
      onPress={() => router.navigate(href)}
    >
      {expanded ? children : undefined}
    </Button>
  );
}
