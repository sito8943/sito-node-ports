use std::sync::OnceLock;

/// Textos del menú de la bandeja. La ventana usa su propio diccionario
/// (src/lang); ambos eligen idioma con la misma regla.
pub struct Texts {
    pub empty: &'static str,
    pub one_process: &'static str,
    /// Se antepone el número: "3 procesos Node".
    pub many_processes: &'static str,
    /// Se le añade la URL: "Abrir http://localhost:3000".
    pub open: &'static str,
    pub kill: &'static str,
    pub show: &'static str,
    pub quit: &'static str,
}

const ES: Texts = Texts {
    empty: "Ningún proceso Node escuchando",
    one_process: "1 proceso Node",
    many_processes: "procesos Node",
    open: "Abrir",
    kill: "Terminar proceso",
    show: "Mostrar ventana",
    quit: "Salir",
};

const EN: Texts = Texts {
    empty: "No Node processes listening",
    one_process: "1 Node process",
    many_processes: "Node processes",
    open: "Open",
    kill: "Terminate process",
    show: "Show window",
    quit: "Quit",
};

/// Primer idioma preferido del sistema con traducción; si ninguno, inglés.
/// Debe coincidir con detectLanguage() en src/lang/index.ts.
fn pick(locales: impl Iterator<Item = String>) -> &'static Texts {
    for locale in locales {
        let code = locale
            .split(['-', '_'])
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        match code.as_str() {
            "es" => return &ES,
            "en" => return &EN,
            _ => {}
        }
    }
    &EN
}

pub fn texts() -> &'static Texts {
    static TEXTS: OnceLock<&'static Texts> = OnceLock::new();
    TEXTS.get_or_init(|| pick(sys_locale::get_locales()))
}

#[cfg(test)]
mod tests {
    use super::pick;

    fn locales(list: &[&str]) -> impl Iterator<Item = String> {
        list.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn picks_first_supported_language() {
        assert_eq!(pick(locales(&["es-ES", "en-US"])).quit, "Salir");
        assert_eq!(pick(locales(&["fr-FR", "es-MX"])).quit, "Salir");
        assert_eq!(pick(locales(&["en_GB"])).quit, "Quit");
        assert_eq!(pick(locales(&["ja-JP"])).quit, "Quit");
    }
}
