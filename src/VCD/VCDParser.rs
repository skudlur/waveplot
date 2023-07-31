use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use vcd::*;
use log::{debug, warn};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum ChangeTypes {
    Bit_1(f64),
    Bit_64(f64),
}

pub fn vcd_parser_wrapper(file_path: &str) -> (Vec<(String, String)>,BTreeMap<u64, Vec<ChangeTypes>>) {
    //==================Variables=================
    let mut i_parser = Parser::new(BufReader::new(File::open(file_path).unwrap()));
    let mut parsed_vcd: BTreeMap<u64, Vec<ChangeTypes>> = BTreeMap::new(); // Output
    let header = i_parser.parse_header().unwrap();
    let mut rolling_hash: HashMap<String, ChangeTypes> = HashMap::new(); // BTreeMap<id, prev_value>
    //===========================================

    //=================Collecting changes in waveplot=======================    
    while let Some(cmd) = i_parser.next().transpose().unwrap() {
        match cmd {
            Command::Timestamp(e) => {
                //If its the first time stamp we can be assured to have all waveform items
                //And it will carry over as we only change if change!
                let mut collected_cache: Vec<ChangeTypes> = Vec::new();
                let values = rolling_hash.values();
                for i in values {
                    collected_cache.push(i.clone()) //Not a worry some clone :)
                }
                parsed_vcd.insert(e, collected_cache);
            }
            Command::ChangeScalar(i, v) => match v {
                Value::V0 => {
                    rolling_hash.insert(i.to_string(), ChangeTypes::Bit_1(0.0));
                }
                Value::V1 => {
                    rolling_hash.insert(i.to_string(), ChangeTypes::Bit_1(1.0));
                }
                _ => {
                    warn!("Found Impedance/unknown value change, ignoring it...");
                }
            },
            Command::ChangeReal(i, v) => {
                rolling_hash.insert(i.to_string(), ChangeTypes::Bit_64(v));
            }
            Command::ChangeString(..) => {
                warn!("Unhandled String vaule changes!")
            }
            _ => {
                debug!("Dono what, but unhandled!");
            }
        }
    }
    //==================================================================================
    
    //---------------------------Header Parsing for map_guide---------------------------
    let mut id_code_reference_map: HashMap<String, &str> = HashMap::new();
    for i in header.items.iter() {
        match i {
            ScopeItem::Scope(s) => {
                for j in s.children.iter() {
                    match j {
                        ScopeItem::Scope(_) => {
                            debug!("2 or more level recursive Scope detected, skipping.");
                        }
                        ScopeItem::Var(v) => {
                            id_code_reference_map.insert(v.code.to_string(), v.reference.as_str());
                        }
                    }
                }
            }
            ScopeItem::Var(v) => {
                id_code_reference_map.insert(v.code.to_string(), v.reference.as_str());
            }
        }
    }
    let mut map_guide: Vec<(String, String)> = Vec::new(); //Vec<waveplot_name> To know which index in array is
                                                           //for which waveplot
    for i in rolling_hash.keys() {
        map_guide.push((
            i.to_string(),
            id_code_reference_map.get(i).unwrap().to_string(),
        ))
    }
    debug!("{:?}", map_guide);
    //--------------------------------------------------------------------------------------------------------
    for (k, v) in parsed_vcd.iter() {
        debug!("{} : {:?}", k, v);
    }

    (map_guide, parsed_vcd)
}
