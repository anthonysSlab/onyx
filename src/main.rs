#![allow(forbidden_lint_groups)]
#![forbid(
    clippy::complexity,
    clippy::suspicious,
    clippy::correctness,
    clippy::cargo,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(
    clippy::style,
    clippy::restriction,
    clippy::match_bool,
    clippy::too_many_lines,
    clippy::single_match_else,
    clippy::ignored_unit_patterns,
    clippy::module_name_repetitions,
    clippy::needless_for_each,
    clippy::derive_partial_eq_without_eq,
    clippy::missing_const_for_fn,
    clippy::cognitive_complexity,
    clippy::option_if_let_else,
    clippy::option_map_unit_fn
)]
#![allow(dead_code, unused)]

use colored::Colorize;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::report::{Level, LogHandler, Report};
use crate::scanner::Scanner;

mod args;
mod ast;
mod lexer;
mod parser;
// mod preprocessor;
mod report;
mod scanner;
mod span;
mod token;

fn main() {
    let args = args::Args::parse(std::env::args().skip(1).collect());

    if *args.debug {
        println!("{args:#?}");
    }

    let handler = LogHandler::new();

    let tokens = {
        let mut lexer = Lexer::new(*args.file, Scanner::get(*args.file), handler.clone());
        lexer.lex_tokens();
        lexer.tokens.move_to_front();

        if *args.debug {
            println!("\n{}", "LEXER".bold());
            lexer.tokens.as_cursor().for_each(|token| println!("{token:#}"));
        }

        if handler.test_ge_log(Level::Warn as u8 as usize) {
            std::process::exit(1);
        }

        lexer.tokens
    };

    // let (tokens, tags) = {
    //     let mut preprocessor =
    //         preprocessor::PreProcessor::new(*args.file, tokens, ReportSender::new(sender.clone()));
    //
    //     let (tokens, tags) = preprocessor.process();
    //
    //     if *args.debug {
    //         println!("\n{}", "PREPROCESSOR".bold());
    //         tokens.iter().for_each(|token| println!("{token:#}"));
    //         println!();
    //         tags.iter().for_each(|tag| println!("{tag:?}"));

    //     }
    //
    //     if check_reports(&receiver, &mut reports) {
    //         print_reports_and_exit(&mut reports, &args);
    //     }
    //
    //     (tokens, tags)
    // };

    let program = {
        let mut parser = Parser::new(&args.file, tokens, handler.clone());
        let result = parser.parse();

        if *args.debug {
            println!("\n{}", "PARSER".bold());
            result.stmts.iter().for_each(|stmt| println!("{stmt:#}"));
        }

        if handler.test_ge_log(Level::Warn as u8 as usize) {
            std::process::exit(1);
        }
    };

    handler.terminate();
}
