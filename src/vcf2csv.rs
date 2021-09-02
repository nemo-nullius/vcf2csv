//use std::collections::HashMap;
use std::collections::HashSet;

/*
pub fn get_vcf_hashmap<'a>(vcf_lines: Vec<&'a str>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut vcf_hashmap: HashMap<&str, Vec<&str>> = HashMap::new();
    //let vcf_lines: Vec<&str> = vcf_raw.lines().collect();
    for (idx, ln) in vcf_lines.iter().enumerate() {
        //let token: &str = ln.lines().collect::<Vec<&str>>()[0];
        /*
        let ln_tokens: Vec<&str> = ln.split(":").collect();
        let (header, content) = match ln_tokens.len() {
            0 => panic!("Bad line at {}", idx),
            1 => (ln, "<EMPTY>"),
            x if x >= 2 => (ln, ln_tokens[1..].join(":").as_str()),
            _ => unreachable!(),
        };
        */
        let token_pos = ln.find(":");
        //TODO: parse photo line to exclude all the skip conditions.
        let (header, content): (&str, &str) = match token_pos {
            None => {
                if ln.len() == 0 {
                    println!("[SKIP EMPTY LINE]");
                    continue;
                }
                if ln.find(" ") == Some(0) {
                    println!("[SKIP PHOTO LINES]: {}", ln);
                    continue;
                }
                panic!("Bad line at {}", idx);
            }
            Some(x) => match x {
                _a if ln.find("PHOTO") == Some(0) => {
                    println!("[SKIP PHOTO FIRST LINE]: {}", ln);
                    continue;
                }
                y if y == ln.len() - 1 => (&ln[..x], "<EMPTY>"),
                _ => (&ln[..x], &ln[x + 1..]),
            },
        };
        let v: &mut Vec<&str> = vcf_hashmap.entry(header).or_insert(vec![]);
        v.push(content);
    }
    vcf_hashmap
}
*/

pub fn get_vcf_headers<'a>(vcf_lines: &'a Vec<&'a str>) -> Result<Vec<&'a str>, &'static str> {
    let mut vcf_headers_hs: HashSet<&str> = HashSet::new();
    let mut entry_sum: u32 = 0;
    //let vcf_lines: Vec<&str> = vcf_raw.lines().collect();
    for (idx, ln) in vcf_lines.iter().enumerate() {
        let token_pos = ln.find(":");
        //TODO: parse photo line to exclude all the skip conditions.
        let (header, _): (&str, &str) = match token_pos {
            None => {
                if ln.len() == 0 {
                    println!("[SKIP EMPTY LINE]");
                    continue;
                }
                if ln.find(" ") == Some(0) {
                    println!("[SKIP PHOTO LINES]: {}", ln);
                    continue;
                }
                //panic!("Bad line at {}", idx);
                println!(
                    "[ABORT] Bad line at {}.\n{}\nIs this a valid vcf file?",
                    idx, ln
                );
                return Err("Bad file format.");
            }
            Some(x) => match x {
                _a if ln.find("PHOTO") == Some(0) => {
                    println!("[SKIP PHOTO FIRST LINE]: {}", ln);
                    continue;
                }
                y if y == ln.len() - 1 => (&ln[..x], "<EMPTY>"),
                _ => {
                    if ln.find("BEGIN") == Some(0) {
                        entry_sum += 1;
                    }
                    (&ln[..x], &ln[x + 1..])
                }
            },
        };
        /*
        if !vcf_headers.iter().any(|&x| x == header) {
            vcf_headers.push(header);
        }
        */
        vcf_headers_hs.insert(header);
    }
    let mut vcf_headers: Vec<&str> = vcf_headers_hs.into_iter().collect();
    println!("{} entries in all.", entry_sum);
    vcf_headers.sort();
    Ok(vcf_headers)
}

pub fn get_csv_body_vec<'a>(
    vcf_lines: &'a Vec<&'a str>,
    vcf_headers: &'a Vec<&'a str>,
) -> Vec<Vec<Vec<&'a str>>> {
    fn renew_csv_entry(csv_entry: &mut Vec<Vec<&str>>, vcf_headers: &Vec<&str>) {
        //TODO: USE ITERATOR
        *csv_entry = Vec::new();
        for _ in vcf_headers {
            (*csv_entry).push(vec![]);
        }
    }
    let mut csv_body_vec: Vec<Vec<Vec<&str>>> = Vec::new();
    //let vcf_headers_hs: HashSet<&str> = HashSet::from_iter(vcf_headers.iter().cloned()); // used for check existance
    let mut csv_entry: Vec<Vec<&str>> = Vec::new();
    renew_csv_entry(&mut csv_entry, vcf_headers);
    let mut endflag: bool = false;

    for (_, ln) in vcf_lines.iter().enumerate() {
        let token_pos = ln.find(":");
        let (header, content): (&str, &str) = match token_pos {
            None => {
                continue;
            }
            Some(x) => match x {
                y if y == ln.len() - 1 => (&ln[..x], ""),
                _ => {
                    if ln.find("BEGIN") == Some(0) {}
                    if ln.find("END") == Some(0) {
                        endflag = true;
                    }
                    (&ln[..x], &ln[x + 1..])
                }
            },
        };
        let header_idx = vcf_headers.iter().position(|&r| r == header);
        match header_idx {
            Some(x) => {
                csv_entry[x].push(content);
            }
            None => {}
        }
        if endflag {
            endflag = false;
            csv_body_vec.push(csv_entry.clone());
            renew_csv_entry(&mut csv_entry, vcf_headers);
            continue;
        }
    }
    csv_body_vec
}

//TODO: USE ITERATOR
pub fn get_csv_body_string(csv_body_vec: &Vec<Vec<Vec<&str>>>) -> String {
    let mut vec_new: Vec<Vec<String>> = Vec::new();
    for entry in csv_body_vec {
        let mut entry_new: Vec<String> = Vec::new();
        for item in entry {
            let pieces = format!("{}{}{}", "\"", item.join("\n"), "\"");
            entry_new.push(pieces);
        }
        vec_new.push(entry_new);
    }

    let mut vec_new2: Vec<String> = Vec::new();
    for entry in vec_new {
        let entry_new = entry.join(",");
        vec_new2.push(entry_new);
    }

    vec_new2.join("\n")
}
