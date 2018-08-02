#[macro_use]
extern crate serde_json;
extern crate search_lib;

#[macro_use] extern crate log;
extern crate env_logger;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::env;
fn main() {
    env_logger::init();
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        panic!("use create oder start_server as argument");
    }
    if args[1] == "create" {
        println!("creating");
        create_jmdict_index();
        // create_suggest_index();
    }else if args[1] == "start_server"{
        info!("starting server");
        // search_lib::server::start_server("jmdict".to_string());
    }else{
        panic!("use create oder start_server as argument");
    }

}


fn create_jmdict_index() -> Result<(), io::Error> {
    let indices = r#"
    [
    {
        "boost": "commonness",
        "options": { "boost_type": "int" }
    },
    { "fulltext": "kanji[].text" },
    { "fulltext": "kana[].text" },
    {
        "fulltext": "meanings.ger[].text",
        "options": { "tokenize": true  }
    },
    {
        "boost": "meanings.ger[].rank",
        "options": { "boost_type": "int" }
    },
    {
        "fulltext": "meanings.eng[]",
        "options": { "tokenize": true  }
    },
    {
        "boost": "kanji[].commonness",
        "options": { "boost_type": "int" }
    },
    {
        "boost": "kana[].commonness",
        "options": { "boost_type": "int" }
    }
    ]
    "#;
    // let mut f = File::open("jmdict_split.json")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // println!("{:?}", search_lib::create::create_indices("jmdict", &s,  indices));

    // create_indices_from_file(
    //     persistence: &mut Persistence,
    //     data_path: &str,
    //     indices: &str,
    //     create_cache: Option<CreateCache>,
    //     load_persistence: bool,
    // )
    search_lib::create::create_indices_from_file(
        &mut search_lib::persistence::Persistence::create("jmdict".to_string()).unwrap(),
        "jmdict_split.json",
        indices,
        None,
        false,
    ).unwrap();


    {
        let mut pers = search_lib::persistence::Persistence::load("jmdict".to_string()).expect("Could not load persistence");
        let config = json!({"path": "meanings.ger[].text"});
        let mut f = File::open("deWords.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        search_lib::create::add_token_values_to_tokens(&mut pers, &s, &config.to_string()).expect("Could not add token values");

        let config = json!({"path": "meanings.eng[]"});
        let mut f = File::open("enWords.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        search_lib::create::add_token_values_to_tokens(&mut pers, &s, &config.to_string()).expect("Could not add token values");

    }


    Ok(())
}



// fn create_suggest_index() -> Result<(), io::Error> {
//     let indices = r#"
//     [
//         {
//             "boost": "commonness",
//             "options": { "boost_type": "int" }
//         },
//         { "fulltext": "text" }
//     ]
//     "#;
//     let mut f = File::open("deWords.json")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     println!("{:?}", search_lib::create::create_indices("desuggest", &s,  indices));


//     let mut f = File::open("engWords.json")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     println!("{:?}", search_lib::create::create_indices("engsuggest", &s,  indices));


//     Ok(())
// }
