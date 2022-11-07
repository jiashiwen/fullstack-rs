use crate::cmd::{new_config_cmd, new_start_cmd, new_stop_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;
use crate::configure::{generate_default_config, set_config_file_path};
use crate::configure::{get_config, get_config_file_path, get_current_config_yml, set_config};
use crate::resources::init_redis_pool;
use crate::{httpserver, interact};
use clap::{Arg, ArgAction, ArgMatches};
use fork::{daemon, Fork};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::net::{self, IpAddr, SocketAddr};
use std::process::Command;
use std::str::FromStr;
use std::{env, fs, thread};
use sysinfo::{Pid, ProcessExt, RefreshKind, System, SystemExt};
use tokio::runtime::Runtime;

lazy_static! {
    static ref CLIAPP: clap::Command = clap::Command::new("serverframe-rs")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("RustBoot")
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .action(ArgAction::SetTrue)
                .help("run as interact mod")
        )
        .subcommand(
            new_start_cmd().arg(
                Arg::new("daemon")
                    .short('d')
                    .long("daemon")
                    .action(ArgAction::SetTrue)
                    .help("run as daemon")
            )
        )
        .subcommand(new_stop_cmd())
        .subcommand(new_config_cmd());
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    let matches = CLIAPP.clone().get_matches();
    cmd_match(&matches);
}

pub fn run_from(args: Vec<String>) {
    match clap::Command::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &clap::Command, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
        } else {
            if beginlevel == 0 {
                all_subcommand(iterm, nextlevel, input);
            }
        }
    }
    let subcommand = SubCmd {
        level: beginlevel,
        command_name: app.get_name().to_string(),
        subcommands: subcmds,
    };
    input.push(subcommand);
}

pub fn get_command_completer() -> CommandCompleter {
    CommandCompleter::new(SUBCMDS.to_vec())
}

fn subcommands() -> Vec<SubCmd> {
    let mut subcmds = vec![];
    all_subcommand(&CLIAPP.clone().borrow(), 0, &mut subcmds);
    subcmds
}

fn cmd_match(matches: &ArgMatches) {
    if let Some(c) = matches.get_one::<String>("config") {
        set_config_file_path(c.to_string());
        set_config(&get_config_file_path());
    } else {
        set_config("");
    }

    if matches.get_flag("interact") {
        interact::run();
        return;
    }

    if let Some(ref matches) = matches.subcommand_matches("start") {
        if matches.get_flag("daemon") {
            let args: Vec<String> = env::args().collect();
            if let Ok(Fork::Child) = daemon(true, true) {
                // Start child thread
                let mut cmd = Command::new(&args[0]);
                for idx in 1..args.len() {
                    let arg = args.get(idx).expect("get cmd arg error!");
                    // remove start as daemon variable
                    // 去除后台启动参数,避免重复启动
                    if arg.eq("-d") || arg.eq("-daemon") {
                        continue;
                    }
                    cmd.arg(arg);
                }

                let child = cmd.spawn().expect("Child process failed to start.");
                fs::write("pid", child.id().to_string()).expect("Write pid file error!");
            }
            println!("{}", "daemon mod");
            std::process::exit(0);
        }

        let banner =
            "                                                                                      
                    .                                                                 
                .'ck0Oo;.                                         .'.                 
             .;d0NMMMMMWKxc'                                     .dNk.                
         .'lkKWMMMMMMMMMMW0o.                                    .dWWKc               
      .;o0NMMMMMMMMMMWXOo;.                                       .codc.    ......    
   .:xKWMMMMMMMMMMMM0c.             .                       ...';:clodxkkOO00KKXXKc.  
   '0MMMMMMMMMMMMMMWo            'cxO;               'cloxkOOOOOk0NMNOolcccccllodko.  
   '0MMMMMMWWMMMMMMWd        .;oONMMN:               .dkdlc;'...'xX0:.  ..            
   '0MMMNOocOMMMMMMWd       .dWMMMMMNc                         :0Ko.  .dX0c.          
   'OKxc'   dWMMMMMWd       .kMMMMMMNc                      .,kNNx:;:o0NOo,           
    ..      dWMMMMMWo       .kMMMMMMNc                    .o0NWWX0KWWXx;.             
            dWMMMMMWo       .kMMMMMMNc                     ;dl:,,o00o'                
            dWMMMMMWo       .kMMMMMMNc                        .;k0o.   .::;;.         
            dWMMMMMWo       .kMMMMMMNc                      .:ONO:',:lkk:..,xo.       
            dWMMMMMWo       .kMMMMMMNc                   ;dkKWMWX0KXK0O:..:xXX:       
            dWMMMMMWo       .kMMMMNOl.                   ,OX0kdlc;'...   .:xKXl       
            dWMMMMMWo       .kWKxc'                       ...               ...       
            cKWMMMMWo        ,;.                                                      
             .:d0NMWo                                                                 
                .,ld;                                                                 
                                                                                      ";
        println!("{}", banner);
        println!("current pid is:{}", std::process::id());
        init_redis_pool();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let async_http_server = async {
            let config = get_config().unwrap();
            let bind = config.http.bind;
            let port = config.http.port;

            let mut http_server = httpserver::HttpServer::default();
            let ip = IpAddr::from_str(&bind).unwrap();
            let addr = net::SocketAddr::from((ip, port));

            http_server.addr = addr;

            let http_handler = http_server.run(rx).await;
            let _http = tokio::join!(http_handler);
        };

        let thread_http = thread::spawn(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async_http_server);
        });
        thread_http.join().unwrap();
    }

    if let Some(ref _matches) = matches.subcommand_matches("stop") {
        println!("server stopping...");
        // let sys = System::new_with_specifics(RefreshKind::with_processes(Default::default()));
        let sys = System::new_with_specifics(RefreshKind::everything().without_disks_list());

        let pidstr = String::from_utf8(fs::read("pid").unwrap()).unwrap();
        let pid = Pid::from_str(pidstr.as_str()).unwrap();

        if let Some(p) = sys.process(pid) {
            println!("terminal process: {:?}", p.pid());
        } else {
            println!("Server not run!");
            return;
        };
        Command::new("kill")
            .args(["-15", pidstr.as_str()])
            .output()
            .expect("failed to execute process");
    }

    if let Some(config) = matches.subcommand_matches("config") {
        if let Some(_show) = config.subcommand_matches("show") {
            let yml = get_current_config_yml();
            match yml {
                Ok(str) => {
                    println!("{}", str);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }

        if let Some(gen_config) = config.subcommand_matches("gendefault") {
            let mut file = String::from("");
            if let Some(path) = gen_config.get_one::<&str>("filepath") {
                file.push_str(path);
            } else {
                file.push_str("config_default.yml")
            }
            if let Err(e) = generate_default_config(file.as_str()) {
                log::error!("{}", e);
                return;
            };
            println!("{} created!", file);
        }
    }
}
