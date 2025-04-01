use parser::parse_hosts;

mod parser;

fn main() {
  let result = parse_hosts(
    r#"192.168.1.1 router.home
127.0.0.1 localhost
# OPTI-HOSTS example.com [Beijing, HK * 2, AS6939]
# OPTI-HOSTS rua.sh [Alibaba * 2]
127.0.0.2 notlocalhost
# invalid below
# OPTI-HOSTS rua.sh [Tencent * 0]"#,
  );
  println!("{:#?}", result)
}
