extern crate native_search;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        panic!("use create oder start_server as argument");
    }
    if args[1] == "create" {
        println!("creating");
        create_jmdict_index();
    }else if args[1] == "start_server"{
        println!("starting server");
        native_search::server::start_server("jmdict".to_string());
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
    let mut f = File::open("jmdict.json")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("{:?}", native_search::create::create_indices("jmdict", &s,  indices));
    Ok(())
}
