use testcontainers::Container;
use testcontainers_modules::{postgres::{self, Postgres}, testcontainers::runners::SyncRunner};

pub fn setup_test_postgres() -> (Container<Postgres>, String) {
    let container = postgres::Postgres::default().start().unwrap();
    let host_port = container.get_host_port_ipv4(5432).unwrap();
    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",
    );

    (container, connection_string.to_string())
}
