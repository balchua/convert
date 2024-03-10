use std::fs::OpenOptions;
use std::io::{prelude::*, BufWriter};
use std::{fs::File, io::BufReader};

use xml::{reader::XmlEvent, ParserConfig};

use crate::domain::acct::Acct;
use crate::domain::balance::Balance;
use crate::domain::grp_hdr::GrpHdr;
use crate::domain::ntry::Ntry;

mod domain;

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Stmt {
//     id: String,
//     elctrnc_seq_nb: String,
//     lgl_seq_nb: String,
//     cre_dt_tm: String,
//     fr_to_dt: FrToDt,
//     acct: Option<Acct>,
//     bal: Option<Vec<Bal>>,
//     txs_summry: Option<TxsSummry>,
//     ntry: Option<Vec<Ntry>>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct FrToDt {
//     fr_dt_tm: String,
//     to_dt_tm: String,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct TxsSummry {
//     ttl_ntries: TtlNtries,
//     ttl_cdt_ntries: TtlCdtNtries,
//     ttl_dbt_ntries: TtlDbtNtries,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct TtlNtries {
//     nb_of_ntries: String,
//     sum: String,
//     ttl_net_ntry_amt: String,
//     cdt_dbt_ind: String,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct TtlCdtNtries {
//     nb_of_ntries: String,
//     sum: String,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct TtlDbtNtries {
//     nb_of_ntries: String,
//     sum: String,
// }

fn main() {
    // Open mt940 file
    let mt940_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("samples/out.mt940")
        .unwrap();
    let mut mt940_file = BufWriter::new(mt940_file);
    // Open the XML file
    let file = File::open("samples/test.xml").expect("Unable to open file");
    let file = BufReader::new(file); // Buffering is important for performance
    let reader = ParserConfig::default()
        .ignore_root_level_whitespace(false)
        .create_reader(BufReader::new(file));
    let mut depth = 0;
    let mut xml = String::new();
    let mut ntry_count = 0;
    let mut balances: Vec<Balance> = Vec::new();
    let mut balance: Balance;
    for e in reader {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "GrpHdr"
                    || name.local_name == "Ntry"
                    || name.local_name == "Acct"
                    || name.local_name == "Bal"
                {
                    depth += 1;
                }

                if depth == 1 {
                    xml.push_str("<");
                    xml.push_str(&name.local_name);
                    for attr in attributes {
                        xml.push_str(&format!(" {}=\"{}\"", &attr.name.local_name, &attr.value));
                    }
                    xml.push_str(">");
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                if depth == 1 {
                    xml.push_str(&s);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if depth == 1 {
                    xml.push_str(&format!("</{}>", &name.local_name));
                }
                if name.local_name == "GrpHdr" {
                    depth -= 1;
                    let grp_hdr: GrpHdr = serde_xml_rs::from_str(&xml).unwrap();
                    let header = grp_hdr.to_mt940_header();
                    writeln!(mt940_file, "{}", header).unwrap();
                    xml.clear();
                }
                if name.local_name == "Ntry" {
                    depth -= 1;
                    let ntry: Ntry = serde_xml_rs::from_str(&xml).unwrap();
                    let transactions = ntry.to_mt940_transactions();
                    writeln!(mt940_file, "{}", transactions).unwrap();
                    ntry_count += 1;
                    xml.clear();
                }
                if name.local_name == "Acct" {
                    depth -= 1;
                    let acct: Acct = serde_xml_rs::from_str(&xml).unwrap();
                    let tag25 = acct.to_mt940_tag25();
                    writeln!(mt940_file, "{}", tag25).unwrap();
                    xml.clear();
                }

                if name.local_name == "Bal" {
                    depth -= 1;
                    balance = serde_xml_rs::from_str(&xml).unwrap();
                    balances.push(balance.clone());
                    let tag25 = balance.to_mt940_opening_balance();
                    if tag25 != "" {
                        writeln!(mt940_file, "{}", tag25).unwrap();
                    }
                    xml.clear();
                }
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    println!("Ntry count: {:?}", ntry_count);
}
