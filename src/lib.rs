use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern]
fn do_cert_check(certno: String) -> bool {
    fn converter(a: &str) -> u32 {
        let b = a.parse::<u32>();
        match b {
            Ok(c) => c,
            _ => 10,
        }
    }

    if certno.len() == 18 {
        let num18 = converter(&certno[0..1]);
        let num17 = converter(&certno[1..2]);
        let num16 = converter(&certno[2..3]);
        let num15 = converter(&certno[3..4]);
        let num14 = converter(&certno[4..5]);
        let num13 = converter(&certno[5..6]);
        let num12 = converter(&certno[6..7]);
        let num11 = converter(&certno[7..8]);
        let num10 = converter(&certno[8..9]);
        let num09 = converter(&certno[9..10]);
        let num08 = converter(&certno[10..11]);
        let num07 = converter(&certno[11..12]);
        let num06 = converter(&certno[12..13]);
        let num05 = converter(&certno[13..14]);
        let num04 = converter(&certno[14..15]);
        let num03 = converter(&certno[15..16]);
        let num02 = converter(&certno[16..17]);
        let num01 = converter(&certno[17..18]);
        let sum = (num18 * 7
            + num17 * 9
            + num16 * 10
            + num15 * 5
            + num14 * 8
            + num13 * 4
            + num12 * 2
            + num11 * 1
            + num10 * 6
            + num09 * 3
            + num08 * 7
            + num07 * 9
            + num06 * 10
            + num05 * 5
            + num04 * 8
            + num03 * 4
            + num02 * 2
            + num01 * 1)
            .rem_euclid(11);
        if sum == 1 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_cert_check() {
        assert_eq!("Hello, cert_check", crate::hello_cert_check());
    }

    #[pg_test]
    fn test_do_cert_check() {
        let cert1 = String::from("513424199612160010");
        assert_eq!(true, crate::do_cert_check(cert1));
        
        let cert2 = String::from("51342419961216001X");
        assert_eq!(false, crate::do_cert_check(cert2));

        let cert3 = String::from("51342420180926004X");
        assert_eq!(true, crate::do_cert_check(cert3));

        let cert4 = String::from("51342420180926014X");
        assert_eq!(false, crate::do_cert_check(cert4));

        let cert5 = String::from("51342420180926004M");
        assert_eq!(true, crate::do_cert_check(cert5));
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts.
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests.
        vec![]
    }
}
