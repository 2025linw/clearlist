export const BUTTON_SIZE = 70;
export const ICON_SIZE = 45;

export const ACTION_BUTTON_SIZE = 55;
export const ACTION_ICON_SIZE = 25;

// Gap between the main button and an action button.
export const ACTION_DISTANCE = 50;

// Distance between their centers.
export const ACTION_OFFSET =
  BUTTON_SIZE / 2 + ACTION_DISTANCE + ACTION_BUTTON_SIZE / 2;

// Space for actions positioned left or above the main button.
export const MENU_SIZE = Math.max(
  BUTTON_SIZE,
  BUTTON_SIZE / 2 + ACTION_OFFSET + ACTION_BUTTON_SIZE / 2,
);
