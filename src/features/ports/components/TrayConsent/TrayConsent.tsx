import { t } from "@/lang";
import { Button } from "@/shared/components/elements/Button";
import { BUTTON_COLOR, BUTTON_SIZE, BUTTON_VARIANT } from "@/shared/constants";
import { TrayConsentProps } from "./types";

import "@/styles/components/TrayConsent.css";

/** Primera pantalla: pide permiso antes de instalarse en la barra de menús. */
export function TrayConsent({ onAccept }: TrayConsentProps) {
  return (
    <section className="TrayConsent">
      <h1>{t("consentTitle")}</h1>
      <p>{t("consentText")}</p>
      <p className="hint">{t("consentHint")}</p>
      <Button
        color={BUTTON_COLOR.PRIMARY}
        variant={BUTTON_VARIANT.SUBMIT}
        size={BUTTON_SIZE.LG}
        onClick={onAccept}
      >
        {t("consentAccept")}
      </Button>
    </section>
  );
}
