import { Href, useRouter } from 'expo-router';

import Icon, { IconColor, IconName } from '@/components/icon';
import Button from '@/components/primitives/button';

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
      icon={
        <Icon
          name={props.iconName || 'list-outline'}
          color={props.iconColor || 'gray'}
        />
      }
      onPress={() => router.replace(href)}
    >
      {expanded ? children : undefined}
    </Button>
  );
}
