import { t } from "@/lang";
import { ERROR_CODE } from "./constants";

export function translateError(code: string): string {
  switch (code) {
    case ERROR_CODE.PROCESS_NOT_FOUND:
      return t("errorProcessNotFound");
    case ERROR_CODE.KILL_FAILED:
      return t("errorKillFailed");
    default:
      return `${t("errorGeneric")} ${code}`;
  }
}
