mod formatter {
    use std::collections::HashMap;

    type Data = HashMap<String, u32>;

    trait Formatter {
        fn format(&self, data: &Data, buf: &mut String);
    }

    struct Report;

    impl Report {
        fn generate(g: impl Formatter, s: &mut String) {
            // backend operations
            let mut data = HashMap::new();
            data.insert("one".to_string(), 1);
            data.insert("two".to_string(), 2);
            // generate report
            g.format(&data, s);
        }
    }

    struct Text;
    impl Formatter for Text {
        fn format(&self, data: &Data, buf: &mut String) {
            for (k, v) in data {
                let entry = format!("{k} {v}\n");
                buf.push_str(&entry);
            }
        }
    }

    struct Json;

    impl Formatter for Json {
        fn format(&self, data: &Data, buf: &mut String) {
            buf.push('[');
            for (k, v) in data.into_iter() {
                let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
                buf.push_str(&entry);
                buf.push(',');
            }
            if !data.is_empty() {
                buf.pop(); // remove extra , at the end
            }
            buf.push(']');
        }
    }

    pub fn test() {
        let mut s = String::from("");
        Report::generate(Text, &mut s);
        println!("{s}");

        s.clear();
        Report::generate(Json, &mut s);
        println!("{s}");
    }
}

mod adder {
    struct Adder;
    impl Adder {
        pub fn add<F>(x: u8, y: u8, f: F) -> u8
        where
            F: Fn(u8, u8) -> u8,
        {
            f(x, y)
        }
    }

    pub fn test() {
        let arith_adder = |x, y| x + y;
        let bool_adder = |x, y| {
            if x == 1 || y == 1 {
                1
            } else {
                0
            }
        };
        let custom_adder = |x, y| 2 * x + y;
        println!("{}", Adder::add(4, 5, arith_adder));
        println!("{}", Adder::add(0, 0, bool_adder));
        println!("{}", Adder::add(1, 3, custom_adder));
    }
}

fn main() {
    formatter::test();
    println!();
    adder::test();
}
