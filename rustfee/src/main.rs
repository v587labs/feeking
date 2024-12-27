

fn main() {
   // 创建命令行对象
    let matches = clap::App::new("rustfee")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(commands::run_subcommand())
        .subcommand(
           clap::SubCommand::with_name("init")
        )
        .get_matches();
   // 匹配参数
   match matches.subcommand() {
      ("run", Some(cmd)) => println!("Running: {:?}", cmd),
      ("init", Some(cmd)) => println!("Initializing: {:?}", cmd),
      _ => unreachable!(),
   }
}
