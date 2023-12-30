// use clap::Parser;
// use futures::executor::block_on;
// use kidsnote_sdk::{KidsnoteSdk, config::KidsnoteConfig};

// #[derive(Parser, Debug)]
// pub struct AuthArgs {
//     /// Client ID
//     #[arg(short = 'c', long = "client_id", env="KNB_CLIENT_ID", value_name = "Client id")]
//     pub client_id: Option<String>,

//     /// Refresh token of the Account to greet
//     #[arg(short = 't', long = "token", env="KNB_REFRESH_TOKEN", value_name = "Refresh token")]
//     pub token: String,
// }


// pub struct AuthCommand {

// }

// impl AuthCommand {

//     /// init and run
//     pub fn run(args:&AuthArgs){ 
//         let inst = Self {  };
//         inst.internal_run(args);
//     }

//     ///
//     fn internal_run(&self, args:&AuthArgs) {
//         let kidsnote_options = KidsnoteConfig::new(args.client_id.clone());
//         let mut kidsnote_sdk = KidsnoteSdk::new(kidsnote_options);

//         let result = block_on(kidsnote_sdk.auth().login(args.user_id.as_str(), args.user_pass.as_str()));
//         match result {
//             Ok(result) => {
//                 println!("{:?}", result);
//             },
//             Err(err) => {
//                 eprintln!("Error: {}", err);
//             }
//         }
//     }
    
// }
