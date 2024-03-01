use std::{collections::HashMap, fs, time::Duration};

use hurl::{
    runner,
    runner::RunnerOptionsBuilder,
    util::{logger::LoggerOptionsBuilder, path::ContextDir},
};
use hurl_core::ast::Retry;
use rstest::*;

#[rstest]
#[case("tests/test.hurl")]
fn test_hurl(#[case] hurl_file: &str) {
    let filename = "-";
    let variables = HashMap::default();
    let logger_opts = LoggerOptionsBuilder::new()
        .color(false)
        .filename(filename)
        .verbosity(None)
        .build();
    let runner_opts = RunnerOptionsBuilder::new()
        .aws_sigv4(None)
        .cacert_file(None)
        .compressed(false)
        .connect_timeout(Duration::from_secs(300))
        .context_dir(&ContextDir::default())
        .cookie_input_file(None)
        .fail_fast(false)
        .follow_location(false)
        .ignore_asserts(false)
        .insecure(false)
        .max_redirect(None)
        .no_proxy(None)
        .post_entry(None)
        .pre_entry(None)
        .proxy(None)
        .retry(Retry::None)
        .retry_interval(Duration::from_secs(1))
        .timeout(Duration::from_secs(300))
        .to_entry(None)
        .unix_socket(None)
        .user(None)
        .user_agent(None)
        .build();

    let content = fs::read_to_string(hurl_file).unwrap();

    let result = runner::run(&content, &runner_opts, &variables, &logger_opts).unwrap();
    assert!(result.success);
}
