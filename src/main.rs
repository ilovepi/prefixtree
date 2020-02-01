extern crate prefixtree;
use env_logger;
use log;
use prefixtree::PrefixTreeRoot;
use pretty_env_logger;
use simple_logger;
use slog_envlogger;
use slog_stdlog;

fn main() {
    //env_logger::init();
    //simple_logger::init();
    //pretty_env_logger::init();
    let _guard = slog_envlogger::init();

    let word = "abcd";
    let mut p = PrefixTreeRoot::create(word).unwrap();
    p.insert("foobar");
    p.debug();
    p.search("");
}
