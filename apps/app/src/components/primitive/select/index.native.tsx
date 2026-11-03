import { BottomSheet, Picker } from '@expo/ui';
import { useState } from 'react';
import { FlatList, Modal, StyleSheet, View } from 'react-native';

import Layout from '@components/layout';
import Button from '@components/primitive/button';
import Icon from '@components/primitive/icon';
import Typography from '@components/primitive/typography';

import { type SelectProps, type SelectValue } from './types';

export default function Select({
  data,
  value,
  defaultValue,
  onValueChange,
  placeholder,
  label,
  type = 'drawer',
}: SelectProps) {
  const [internalValue, setInternalValue] = useState<SelectValue | ''>(
    defaultValue ?? '',
  );
  const [isPresented, setIsPresented] = useState(false);

  const isControlled = value !== undefined;
  const selectedValue = isControlled ? value : internalValue;

  function handleChange(nextValue: SelectValue | '') {
    if (!isControlled) setInternalValue(nextValue);

    onValueChange?.(nextValue);
  }

  return (
    <>
      <SelectButton
        field={label}
        value={selectedValue}
        onPress={() => {
          setIsPresented(true);
        }}
      />

      {type === 'drawer' ? (
        <BottomSheet
          isPresented={isPresented}
          onDismiss={() => setIsPresented(false)}
        >
          <Picker
            selectedValue={selectedValue}
            onValueChange={handleChange}
            appearance="wheel"
          >
            <Picker.Item
              label={placeholder ?? '-'}
              value=""
            />

            {data.map(({ label, value }) => (
              <Picker.Item
                key={value}
                label={label ?? value}
                value={value}
              />
            ))}
          </Picker>
        </BottomSheet>
      ) : (
        <Modal
          visible={isPresented}
          presentationStyle="fullScreen"
        >
          <Layout>
            <FlatList
              data={data}
              renderItem={({ item }) => (
                <Button
                  scheme="tertiary"
                  hasBorder
                  onPress={() => {
                    handleChange(item.value);

                    setIsPresented(false);
                  }}
                >
                  {item.label}
                </Button>
              )}
            />
          </Layout>
        </Modal>
      )}
    </>
  );
}

type SelectButtonProps = {
  field?: string;
  label?: string;
  value?: SelectValue;
  onPress?: () => void;
};

function SelectButton({ field, label, value, onPress }: SelectButtonProps) {
  const styles = StyleSheet.create({
    content: {
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'space-between',
    },
  });

  // Convert from SelectValue to string;
  const resolvedValue = label || String(value);

  return (
    <Button
      scheme="tertiary"
      hasBorder
      rightIcon={<Icon name="chevron-forward" />}
      onPress={onPress}
    >
      <View style={styles.content}>
        <Typography>{field}</Typography>

        <Typography palette="subtle">{resolvedValue}</Typography>
      </View>
    </Button>
  );
}
