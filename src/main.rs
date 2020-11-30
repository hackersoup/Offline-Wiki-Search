#![allow(unused_variables)]

use std::fs::File;
use std::path::PathBuf;
use std::io::{ BufReader, BufRead };
use bzip2::read::MultiBzDecoder;
use structopt::StructOpt;
use levenshtein::levenshtein;

#[derive(Debug, StructOpt)]
#[structopt(name = "Test", about = "Usage example")]
struct Arguments {
    #[structopt(short, long, parse(from_os_str))]
    index: PathBuf,

    #[structopt(short, long)]
    query: String,
}

#[derive(Debug)]
struct LineData {
    bzip_offset: usize,
    article_id: usize,
    article_title: String,
    query_distance: usize,
}

fn main() {
    let opt: Arguments = Arguments::from_args();

    //let wikifile = File::open(dumpfile_path)
    //    .expect("Could not open wiki dump archive");
    let indexfile = File::open(&opt.index)
        .expect("Could not open wiki index file");

    //let wikidecoder = MultiBzDecoder::new(wikifile);

    //let indexreader = BufReader::new(MultiBzDecoder::new(indexfile));
    let indexreader = BufReader::new(File::open(&opt.index).unwrap());

    let query_lower = opt.query.to_lowercase();

    let mut search_data = indexreader.lines()
        .map(|line| { // Convert each line into a useful data structure
            let mut fields = line.as_ref().unwrap().split(':');

            let bzip_offset = fields.next().unwrap().parse::<u64>().unwrap();
            let article_id = fields.next().unwrap().parse::<u64>().unwrap();
            let article_title = fields.next().unwrap();

            LineData {
                bzip_offset: bzip_offset as usize,
                article_id: article_id as usize,
                article_title: String::from(article_title),
                query_distance: levenshtein(article_title, query_lower.as_str())
            }
        })
        .filter(|linedata|  // Filter the data structure based on whether it contains our query
            linedata.article_title.to_lowercase().contains(&query_lower))
        .collect::<Vec<_>>();

    search_data.sort_by(|a, b| match a.query_distance > b.query_distance {
        true => std::cmp::Ordering::Greater,
        false => std::cmp::Ordering::Less,
    });

    println!("Found {} matches!", search_data.len());

    for search_result in search_data {
        println!("{}\t{}", search_result.query_distance, search_result.article_title);
        break;
    }
}
