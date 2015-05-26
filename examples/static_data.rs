extern crate ucl;

static DOC: &'static str = r#"
param = value;
section {
    flag = true;
    number = 10k;
    subsection {
        hosts = {
            host = "localhost";
            port = 9000
        }
        hosts = {
            host = "remotehost"
            port = 9090
        }
    }
}
"#;

fn main() {
    let parser = ucl::Parser::new();
    let doc = parser.parse(DOC).unwrap();

    println!("{:?}", doc);
    println!("{:?}", doc.fetch("param"));
    println!("{:?}", doc.fetch("param").unwrap().as_string());
}
