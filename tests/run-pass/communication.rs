// ignore-windows: TODO env var emulation stubbed out on Windows
// compile-flags: -Zmiri-enable-communication

fn main() {
    assert_eq!(std::env::var("MIRI_ENV_VAR_TEST"), Ok("0".to_owned()));
}
