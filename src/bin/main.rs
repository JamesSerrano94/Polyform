#[macro_use] extern crate clap;
use clap::Parser;
use blocks::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;
use std::collections::{BTreeMap, BTreeSet};
type Key = BTreeSet<(i32,i32,i32)>;
type Memo = BTreeMap<Key, bool>;
use std::sync::{Arc, Mutex};
use std::{thread, time};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Export {
    Scad,
    Tuples,
    Analysis
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    live: Option<usize>,

    #[arg(short, long)]
    shuffles: Option<usize>,

    #[arg(short, long)]
    length: usize,

    #[arg(short, long)]
    export: Export,

    #[arg(short, long)]
    norender: bool,

    #[arg(short, long)]
    amount: Option<usize>,
}





    
fn main() {
    let args = Args::parse();

    let mut pfm = Polyform::new(args.length);

    // if you specify both, you'll get a pre-shuffled polyform so the less interesting shuffles
    // happen quickly

    if let Some(render_step) = args.live {
        // TODO: don't ignore the export type in render shuffle mode
        pfm.render_shuffle(render_step, args.shuffles);
    } else {
        match args.shuffles {
            Some(shuffles) => {
                match args.amount {
                    Some(amount) => {
                        let mut distribution: HashMap<i32, i32> = HashMap::new();

                        for i in 0..amount{
                            let mut newPfm = Polyform::new(args.length);
                        
                            newPfm.shuffle(shuffles);
                            //newPfm.complex = newPfm.reorient();
                            newPfm.reorient();
                            
                            if !args.norender {

                    // technically does n+1 shuffles, there's an easy fix here but it's not super important
                                println!("{}", match args.export { 
                                    Export::Scad => newPfm.export_scad(),
                                    Export::Tuples => newPfm.export(),
                                    Export::Analysis => newPfm.export_analysis()
                                });
                                newPfm.render_shuffle(1, Some(1));

                            } else {

                                println!("{}", match args.export { 
                                    Export::Scad => newPfm.export_scad(),
                                    Export::Tuples => newPfm.export(),
                                    Export::Analysis => newPfm.export_analysis()
                                });
                                
                                // let keyValue = newPfm.findKeyValue();
                                // if !distribution.contains_key(&keyValue){
                                //     distribution.insert(keyValue, 1);
                                // } else {
                                //     let newValue = distribution.get(&keyValue).unwrap() + 1;
                                //     distribution.insert(keyValue, newValue);
                                // }   
                            }

                        }


                        // for val in distribution.values() {
                        //     println!("{val}");
                        // }
                    },
                    None => {
                        eprintln!("Use --amount <count> to supply the number of polyominos.")
                    }
                }
            },
            None => {
                eprintln!("Use --shuffles <count> to supply the number of shuffles.");
            }
        }
    }
}
