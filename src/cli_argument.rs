use std::env;
use std::result::Result;

use crate::validation::check_is_url;
#[derive(Debug)]
pub struct CliArguments {
    pub link: String,
    pub as_service: bool,
}

impl CliArguments {
    fn new(args: &[String]) -> Result<CliArguments, ()> {
        if args.len() < 2 {
            let link: String = "null".to_string();
            return Ok(CliArguments {
                link: link,
                as_service: true,
            });
        }
        let mut link: String = "null".to_string();
        let mut as_service: bool = true;
        for arg in args {
            if arg == "--no-service" {
                as_service = false;
            }else if check_is_url(arg.to_string()) != "".to_string() {
                link = arg.to_string();
            }
        }
        return Ok(CliArguments {
            link: link,
            as_service: as_service,
        });
    }
}

pub fn arguments() -> CliArguments {
    let _args: Vec<String> = env::args().collect();

    let _arg: CliArguments = CliArguments::new(&_args).expect("IDK why ...");
    return _arg;
}
