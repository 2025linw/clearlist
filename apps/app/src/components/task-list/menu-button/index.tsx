import { type StyleProp, View, type ViewStyle } from 'react-native';

import CardButton from './card-button';
import ListButton from './list-button';

type MenuState = 'list' | 'card';

type MenuButtonProps = {
  state: MenuState;
  style: StyleProp<ViewStyle>;
  deleted: boolean;
  onAddTask?: () => void;
  onTrashTask?: () => void;
  onRestoreTask?: () => void;
};

export default function MenuButton({
  state,
  style,
  ...props
}: MenuButtonProps) {
  return (
    <View style={style}>
      {state === 'list' ? <ListButton {...props} /> : <CardButton {...props} />}
    </View>
  );
}
