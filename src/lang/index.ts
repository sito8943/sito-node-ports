import { en } from "./en";
import { es } from "./es";
import type { Dictionary } from "./types";

export type { Dictionary } from "./types";

export type TranslationKey = keyof Dictionary;

/** Diccionarios disponibles por código de idioma (ISO 639-1). */
const DICTIONARIES: Record<string, Dictionary> = { en, es };
const FALLBACK_LANGUAGE = "en";

/**
 * Primer idioma preferido del sistema que tenga diccionario; si ninguno,
 * inglés. Debe coincidir con la elección del menú en src-tauri/src/i18n.rs.
 */
function detectLanguage(): string {
  const preferred = navigator.languages?.length
    ? navigator.languages
    : [navigator.language];
  for (const tag of preferred) {
    const code = tag.toLowerCase().split("-")[0];
    if (code in DICTIONARIES) return code;
  }
  return FALLBACK_LANGUAGE;
}

export const language = detectLanguage();
const dictionary = DICTIONARIES[language];

/**
 * Las cadenas con datos son funciones, no plantillas con marcadores:
 * así `t` exige los argumentos correctos en tiempo de compilación.
 */
type TranslationArgs<K extends TranslationKey> = Dictionary[K] extends (
  ...args: infer P
) => string
  ? P
  : [];

export const t = <K extends TranslationKey>(
  key: K,
  ...args: TranslationArgs<K>
): string => {
  const entry: unknown = dictionary[key];
  if (typeof entry === "function") {
    return (entry as (...params: TranslationArgs<K>) => string)(...args);
  }
  return entry as string;
};
