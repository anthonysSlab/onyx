#![allow(dead_code, unused)]

use crate::error::{Error, ErrorLevel, ErrorSender, Unbox};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::process::exit;
use std::sync::mpsc::Receiver;

mod args;
mod ast;
mod error;
mod lexer;
mod parser;
mod scanner;
mod span;
mod token;

fn check_reports(receiver: &Receiver<Box<Error>>, reports: &mut Vec<Error>) -> bool {
    let mut had_error = false;
    receiver.try_iter().for_each(|report| {
        if report.level() >= ErrorLevel::Error {
            had_error = true;
        }
        reports.push(report.unbox())
    });
    had_error
}

fn print_reports_and_exit(reports: &mut Vec<Error>, args: &args::Args) {
    if args.level.field.unwrap() == ErrorLevel::Silent {
        exit(1);
    }
    reports.sort_by(|left, right| {
        left.level()
            .partial_cmp(&right.level())
            .expect("Failed to order error kinds.")
    });
    reports.iter().for_each(|report| {
        if args.level.field.unwrap() <= report.level() {
            report.display(args.code_context.field.unwrap());
        }
    });
    exit(1);
}

fn main() {
    let args = args::Args::parse(std::env::args().skip(1).collect()).resolve_defaults();
    println!("{:?}", args);

    let mut reports = Vec::<Error>::new();
    let (sender, receiver) = std::sync::mpsc::channel::<Box<Error>>();

    let filename = args.file.get();

    let tokens = {
        let mut lexer = Lexer::new(
            filename,
            Scanner::get_file(filename),
            ErrorSender::new(sender.clone()),
        );
        lexer.lex_tokens();
        if *args.debug.get() {
            lexer
                .tokens
                .iter()
                .for_each(|token| println!("{:#}", token))
        }
        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        };
        lexer.tokens
    };

    let program = {
        let mut parser = Parser::new(args.file.get(), tokens, ErrorSender::new(sender));
        let result = parser.parse();
        if *args.debug.get() {
            result.stmts.iter().for_each(|stmt| println!("{:#}", stmt))
        }
        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        };
    };
}
