import { Tooltip as UiTooltip } from "@sito/ui";

import { TooltipPropsType } from "./types";

/** Envoltorio de @sito/ui — ver Button para el porqué (§14.1). */
export function Tooltip(props: TooltipPropsType) {
  return <UiTooltip {...props} />;
}
