import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faEyeSlash,
  faPowerOff,
  faXmark,
} from "@fortawesome/free-solid-svg-icons";

import { t } from "@/lang";
import { IconButton } from "@/shared/components/elements/IconButton";
import { Tooltip } from "@/shared/components/elements/Tooltip";
import { Spinner } from "@/shared/components/elements/Spinner";
import {
  BUTTON_COLOR,
  BUTTON_VARIANT,
  SHORTCUT_LABEL,
  TOOLTIP_ALIGN,
  TOOLTIP_SIDE,
} from "@/shared/constants";
import { ProcessCard } from "./components/ProcessCard";
import { TrayConsent } from "./components/TrayConsent";
import { usePorts } from "./providers/PortsProvider";

import "@/styles/views/Ports.css";

export function Ports() {
  const {
    trayEnabled,
    processes,
    busyPid,
    error,
    enableTray,
    disableTray,
    kill,
    open,
    hideWindow,
    clearError,
  } = usePorts();

  if (trayEnabled === null) {
    return (
      <div className="Ports loading">
        <Spinner />
      </div>
    );
  }

  return (
    <div className="Ports">
      {error ? (
        <aside className="error">
          <p>{error}</p>
          <Tooltip
            label={t("dismiss")}
            side={TOOLTIP_SIDE.BOTTOM}
            align={TOOLTIP_ALIGN.END}
          >
            <IconButton
              aria-label={t("dismiss")}
              icon={<FontAwesomeIcon icon={faXmark} />}
              variant={BUTTON_VARIANT.TEXT}
              color={BUTTON_COLOR.ERROR}
              onClick={clearError}
            />
          </Tooltip>
        </aside>
      ) : null}

      {!trayEnabled ? (
        <TrayConsent onAccept={enableTray} />
      ) : (
        <>
          <header>
            <div>
              <h1>{t("processesTitle")}</h1>
              <span className="status">{t("trayActive")}</span>
            </div>
            <div className="header-actions">
              <Tooltip
                label={t("hideWindow")}
                side={TOOLTIP_SIDE.BOTTOM}
                align={TOOLTIP_ALIGN.END}
              >
                <IconButton
                  aria-label={t("hideWindow")}
                  icon={<FontAwesomeIcon icon={faEyeSlash} />}
                  variant={BUTTON_VARIANT.TEXT}
                  onClick={hideWindow}
                />
              </Tooltip>
              <Tooltip
                label={t("trayDisable")}
                side={TOOLTIP_SIDE.BOTTOM}
                align={TOOLTIP_ALIGN.END}
              >
                <IconButton
                  aria-label={t("trayDisable")}
                  icon={<FontAwesomeIcon icon={faPowerOff} />}
                  variant={BUTTON_VARIANT.TEXT}
                  color={BUTTON_COLOR.ERROR}
                  onClick={disableTray}
                />
              </Tooltip>
            </div>
          </header>

          <p className="shortcut-hint">{t("shortcutHint", SHORTCUT_LABEL)}</p>

          {processes.length === 0 ? (
            <div className="empty">
              <p className="empty-title">{t("emptyTitle")}</p>
              <p className="empty-hint">{t("emptyHint")}</p>
            </div>
          ) : (
            <div className="list">
              {processes.map((process) => (
                <ProcessCard
                  key={process.pid}
                  process={process}
                  busy={busyPid === process.pid}
                  onOpen={open}
                  onKill={kill}
                />
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
}
