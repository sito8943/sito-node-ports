import {
  BUTTON_COLOR_VARIANTS,
  BUTTON_SIZES,
  BUTTON_VARIANTS,
  type TooltipAlign,
  type TooltipSide,
} from "@sito/ui";

export const THEME = {
  LIGHT: "light",
  DARK: "dark",
} as const;

export type Theme = (typeof THEME)[keyof typeof THEME];

// Alias de los const-objects de @sito/ui: los features los usan desde aquí y
// no se acoplan a la librería (§14.1).
export const BUTTON_COLOR = BUTTON_COLOR_VARIANTS;
export const BUTTON_VARIANT = BUTTON_VARIANTS;
export const BUTTON_SIZE = BUTTON_SIZES;

// @sito/ui 0.9.0 no exporta const-objects para Tooltip: se tipan contra la lib
// (satisfies) para que tsc avise si la API cambia.
export const TOOLTIP_SIDE = {
  TOP: "top",
  BOTTOM: "bottom",
} as const satisfies Record<string, TooltipSide>;

export const TOOLTIP_ALIGN = {
  START: "start",
  CENTER: "center",
  END: "end",
} as const satisfies Record<string, TooltipAlign>;

// Atajo global que alterna la ventana (TOGGLE_SHORTCUT en src-tauri/src/lib.rs).
export const SHORTCUT_LABEL = "⌃⌥N";
