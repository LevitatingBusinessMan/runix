use spin::Once;
pub static CONFIG: Once<Config> = Once::new();

/// The Runix configuration
#[derive(Clone, Debug)]
pub struct Config {
    /// Enables the red screen of death
    pub panic_cover: bool,
    /// If debug information should be printed at boot
    pub print_info: bool,
    /// If a centered welcome message should be printed at boot
    pub welcome: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            panic_cover: true,
            print_info: false,
            welcome: true,
        }
    }
}

pub(super) fn parse(args: &str) {
    let mut config = Config::default();

    let mut lexeme = [0; 64];
    let mut len = 0;

    let mut negafier = false;

    macro_rules! reset {
        () => {
            lexeme = [0; 64];
            len = 0;
            negafier = false;
        };
    }

    //let mut backslash = false;

    for c in args.chars() {
        if c == ' ' {
            if len != 0 {
                let as_str = core::str::from_utf8(&lexeme[0..len]).expect("Invalid character in lexeme");
                panic!("Unknown lexeme: {:?}", as_str);
            }
            lexeme = [0; 64];
            len = 0;
            continue;
        }
        if len >= 64 {
            panic!("Lexeme longer than 64 characters: {:?}", lexeme);
        }

        lexeme[len] = c as u8;
        len += 1;

        let as_str = core::str::from_utf8(&lexeme[0..len]).expect("Invalid character in lexeme");

        match as_str {
            "no" => {
                lexeme = [0; 64];
                len = 0;
                negafier = !negafier
            },
            "panic_cover" => {
                config.panic_cover = !negafier;
                reset!();
            },
            "print_info" => {
                config.print_info = !negafier;
                reset!();
            },
            "welcome" => {
                config.welcome = !negafier;
                reset!();
            }
            _ => {}
        }
    }

    CONFIG.call_once(|| config);
}
