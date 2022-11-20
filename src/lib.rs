use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_cert_check() -> &'static str {
    "Hello, cert_check"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_cert_check() {
        assert_eq!("Hello, cert_check", crate::hello_cert_check());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
