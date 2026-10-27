import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowUpRightFromSquare,
  faStop,
} from "@fortawesome/free-solid-svg-icons";

import { t } from "@/lang";
import { Button } from "@/shared/components/elements/Button";
import { IconButton } from "@/shared/components/elements/IconButton";
import { Tooltip } from "@/shared/components/elements/Tooltip";
import {
  BUTTON_COLOR,
  BUTTON_SIZE,
  BUTTON_VARIANT,
  TOOLTIP_ALIGN,
  TOOLTIP_SIDE,
} from "@/shared/constants";
import { ProcessCardProps } from "./types";

import "@/styles/components/ProcessCard.css";

export function ProcessCard({
  process,
  busy,
  onOpen,
  onKill,
}: ProcessCardProps) {
  return (
    <article className="ProcessCard">
      <header>
        <h2 title={process.cwd ?? undefined}>{process.label}</h2>
        <span className="meta">
          {process.script ? `${process.script} · ` : ""}
          {t("pidLabel", process.pid)}
        </span>
      </header>
      {process.cwd ? <p className="cwd">{process.cwd}</p> : null}
      <div className="actions">
        <div className="ports">
          {process.ports.map((port) => (
            <Tooltip
              key={port}
              label={t("openPort", port)}
              side={TOOLTIP_SIDE.BOTTOM}
              align={TOOLTIP_ALIGN.START}
            >
              <Button
                size={BUTTON_SIZE.SM}
                variant={BUTTON_VARIANT.OUTLINED}
                aria-label={t("openPort", port)}
                onClick={() => onOpen(port)}
              >
                {`:${port}`} <FontAwesomeIcon icon={faArrowUpRightFromSquare} />
              </Button>
            </Tooltip>
          ))}
        </div>
        <Tooltip
          label={t("kill")}
          side={TOOLTIP_SIDE.BOTTOM}
          align={TOOLTIP_ALIGN.END}
        >
          <IconButton
            aria-label={t("kill")}
            icon={<FontAwesomeIcon icon={faStop} />}
            color={BUTTON_COLOR.ERROR}
            variant={BUTTON_VARIANT.TEXT}
            disabled={busy}
            onClick={() => onKill(process.pid)}
          />
        </Tooltip>
      </div>
    </article>
  );
}
