use crate::report::{Level, ReportKind};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::process::exit;

macro_rules! error {
    ($($ident:tt)*) => {
        ReportKind::ArgumentParserError
            .new(format!($($ident)*))
            .with_note("(Run with \x1b[1m--help\x1b[0m for usage information)")
            .display(false);
        exit(1);
    };
}

#[derive(Default)]
pub struct Arg<T> {
    field: Box<T>,
    name:  &'static str,
    set:   bool,
}

impl<T> Arg<T> {
    pub fn new(default: T, name: &'static str) -> Self {
        Self { 
            field: Box::new(default),
            name,
            set: false,
        }
    }

    pub fn try_mut(&mut self, value: T) {
        if self.set {
            error!("the argument {} cannot be used multiple times", self.name);
        }
        self.field = Box::new(value);
    }

    pub fn get(&self) -> &T {
        &self.field
    }
}

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.field)
    }
}

#[derive(Debug)]
pub struct Args {
    pub file:   Arg<&'static str>,
    pub output: Arg<&'static str>,
    pub debug:  Arg<bool>,
    pub code_context: Arg<bool>,
    pub level:  Arg<Level>,
    pub verbs:  Vec<&'static str>,
}

impl Args {
    pub fn default() -> Self {
        Self {
            file:         Arg::new("main.shd",  "--file"),
            output:       Arg::new("main.asm",  "--output"),
            debug:        Arg::new(false,       "--debug"),
            code_context: Arg::new(true,        "--code-context"),
            level:        Arg::new(Level::Warn, "--error-level"),
            verbs:        Vec::new(),
        }
    }

    fn handle_arg(&mut self, arg: &str, args: &mut std::vec::IntoIter<String>, is_end: bool) {
        match arg {
            "h" => {
                println!("{}", USAGE);
                exit(0);
            },
            "help" => {
                println!("{}\n\n{}", USAGE, HELP_MESSAGE);
                exit(0);
            },
            "V" | "version" => {
                println!("sharc {}", env!("CARGO_PKG_VERSION"));
                exit(0);
            },
            "d" | "debug" => self.debug.try_mut(true),
            "l" | "error-level" => {
                if !is_end {
                    error!("flags with parameters must be at the end of a group, or defined separately");
                };

                let level = args.next().unwrap_or_else(|| {
                    error!("expected level");
                });

                self.level.try_mut(match level.as_str() {
                    "f" | "fatal" => Level::Fatal,
                    "e" | "error" => Level::Error,
                    "w" | "warn" => Level::Warn,
                    "n" | "note" => Level::Note,
                    "s" | "silent" => Level::Silent,
                    _ => {
                        error!("invalid level `{}`", level);
                    },
                });
            },
            "no-context" => self.code_context.try_mut(false),
            "f" | "file" => {
                if !is_end {
                    error!("flags with parameters must be at the end of a group, or defined separately");
                };

                let file = args.next().unwrap_or_else(|| {
                    error!("expected file");
                });

                self.file.try_mut(Box::leak(file.into_boxed_str()));
            },
            "o" | "output" => {
                if !is_end {
                    error!("flags with parameters must be at the end of a group, or defined separately");
                };

                let output = args.next().unwrap_or_else(|| {
                    error!("expected file");
                });

                self.output.try_mut(Box::leak(output.into_boxed_str()));
            },
            _ => {
                error!("unrecognized argument '{}'", arg);
            },
        }
    }

    pub fn parse(args: Vec<String>) -> Self {
        let mut out = Self::default();
        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            if let Some(arg) = arg.strip_prefix("--") {
                out.handle_arg(arg, &mut args, true);
            } 

            else if let Some(arg) = arg.strip_prefix("-") {
                for (i, c) in arg.char_indices().map(|(i, _)| &arg[i..i + 1]).enumerate() {
                    out.handle_arg(c, &mut args, i == arg.len() - 1)
                }
            } 

            else {
                if arg == "shark" {
                    println!("\x1b[34m{}\x1b[0m", SHARK_ASCII);
                    exit(1);
                }

                out.verbs.push(Box::leak(arg.into_boxed_str()) as &str);

                // drain remaining args
                while let Some(arg) = args.next() {
                    out.verbs.push(Box::leak(arg.into_boxed_str()) as &str);
                }
            }
        }
        out
    }
}

const USAGE: &str = "Usage: sharc [-hvd] [-l LEVEL] [-f FILE] [-o FILE] [VERB...]";
const HELP_MESSAGE: &str = "\x1b[1mDESCRIPTION\x1b[0m
    The compiler for the Shard Programming Language.
    Documentation can be found at https://shardlang.org/doc/

\x1b[1mOPTIONS\x1b[0m
    -h, --help                  Show only usage with -h
    -v, --version               Show version
    -d, --debug                 Print debug information
        Shows a ton of information not intended for mere mortals.
    -l, --error-level LEVEL     [fatal|error|warn|note|silent]
        (default: warn)
    -f, --file FILE             File to compile
        (default: main.shd)
    -o, --output FILE           File to write to
        (default: main.asm)

        --no-context            Disable code context";
const SHARK_ASCII: &str = r#"                                 ,-
                               ,'::|
                              /::::|
                            ,'::::o\                                      _..
         ____........-------,..::?88b                                  ,-' /
 _.--"""". . . .      .   .  .  .  ""`-._                           ,-' .;'
<. - :::::o......  ...   . . .. . .  .  .""--._                  ,-'. .;'
 `-._  ` `":`:`:`::||||:::::::::::::::::.:. .  ""--._ ,'|     ,-'.  .;'
     """_=--       //'doo.. ````:`:`::::::::::.:.:.:. .`-`._-'.   .;'
         ""--.__     P(       \               ` ``:`:``:::: .   .;'
                "\""--.:-.     `.                             .:/
                  \. /    `-._   `.""-----.,-..::(--"".\""`.  `:\
                   `P         `-._ \          `-:\          `. `:\
                                   ""            "            `-._)"#;
