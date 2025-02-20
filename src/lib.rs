use bincode;
use object::{
    coff, Object, ObjectSection, ObjectSymbol, RelocationTarget, SymbolScope, SymbolSection,
};
//use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter};

const PROJECT_FILE: &str = "project_data.sohot";

pub fn init(objs: &[String], verbose: bool) -> Result<(), Box<dyn Error>> {
    let mut all_syms = HashMap::new();

    // Save all the symbols in each obj that are defined in it. We don't care about internal
    // symbols (because they'll be regenerated if/as necessary next time) and we don't care about
    // undef externals because they're either not our code, or they're in another object.
    for (obj_index, obj) in objs.iter().enumerate() {
        let data = fs::read(obj)?;
        let file: coff::CoffFile = coff::CoffFile::parse(&*data)?;
        for symbol in file.symbols() {
            let want = match symbol.section() {
                SymbolSection::Unknown => false,
                SymbolSection::None => false,
                SymbolSection::Undefined => false,
                SymbolSection::Absolute => false, // ? not sure
                SymbolSection::Common => panic!("common, dunno {:?}", symbol.name()),
                SymbolSection::Section(_) => match symbol.scope() {
                    SymbolScope::Unknown => false,
                    SymbolScope::Compilation => false, // static
                    SymbolScope::Linkage => true,      // extern
                    SymbolScope::Dynamic => true,      // dllexport
                },
                _ => panic!("unexpected symbol section"),
            };
            if want {
                all_syms.insert(symbol.name()?.to_string(), obj_index);
            }
        }
    }

    let mut f = BufWriter::new(fs::File::create(PROJECT_FILE).unwrap());
    bincode::serialize_into(&mut f, &all_syms).unwrap();

    if verbose {
        println!(
            "sohot: initialized with {} objects, {} syms",
            objs.len(),
            all_syms.len()
        );
    }

    Ok(())
}

pub fn generate_patch_and_update(
    index: usize,
    in_obj: &str,
    _out_obj: &str,
) -> Result<(), Box<dyn Error>> {
    let mut f = BufReader::new(fs::File::open(PROJECT_FILE).unwrap());
    let available_globals: HashMap<String, usize> = bincode::deserialize_from(&mut f).unwrap();

    let data = fs::read(in_obj)?;
    let file: coff::CoffFile = coff::CoffFile::parse(&*data)?;

    println!("--- in_obj symtab");
    for sym in file.symbols() {
        println!("{:?}", sym.name()?);
    }

    println!("--- walking all section relocations");
    for section in file.sections() {
        println!("section {:?}", section.name()?);
        for reloc in section.relocations() {
            let target = reloc.1.target();
            if let RelocationTarget::Symbol(sym_idx) = target {
                let sym = file.symbol_by_index(sym_idx)?;
                let name = sym.name()?;
                if let Some(in_obj) = available_globals.get(name) {
                    if *in_obj != index {
                        println!("reloc to {:?} -- STRIP!", sym.name()?);
                    } else {
                        println!("reloc to {:?} saved but ours, keep", sym.name()?);
                    }
                } else {
                    println!("reloc to {:?} keep", sym.name()?);
                }
            }
        }
    }

    Ok(())
}
