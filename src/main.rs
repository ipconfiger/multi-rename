use std::fs;
use clap::{App, Arg, ArgMatches};
use regex::Regex;

fn split_string_by_delimiters(input: &str) -> Vec<String> {
    let re = Regex::new(r"[\s|,;.\[\]\(\)【】]+").unwrap();
    let splited = re.split(input).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    splited.into_iter().map(|s| s.to_string()).collect()
}
fn list_dir() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if !entry.file_type().unwrap().is_dir() {
                            //println!("{}", entry.path().display());
                            paths.push(entry.path().display().to_string());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading entry: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    };
    paths
}

fn blast_files(files: Vec<String>) {}

pub fn get_matches() -> ArgMatches {
    App::new("Move and Rename")
        .arg(Arg::with_name("select")
            .short('s')
            .long("select")
            .takes_value(true)
            .help("选择文件名片段")
        )
        .arg(Arg::with_name("try")
            .short('t')
            .long("try")
            .action(clap::ArgAction::SetTrue)
            .help("显示要执行的命令"))
        .arg(Arg::with_name("do")
            .short('d')
            .long("do")
            .action(clap::ArgAction::SetTrue)
            .help("正式执行"))
        .get_matches()
}

fn main() {
    let mut results:Vec<Vec<String>> = Vec::new();
    let mut name_seqs: Vec<Vec<String>> = Vec::new();
    let files = list_dir();
    for fname in files.clone() {
        let blasted= split_string_by_delimiters(fname.replace("/", "").as_str());
        name_seqs.push(blasted.clone());
        let show_blasted = blasted.iter().enumerate().map(|(i,s)|{format!("{}:{}", i, s)}).collect::<Vec<String>>().join(" ");
        results.push(vec![fname.clone(), show_blasted.clone()])
    }
    let matchs = get_matches();

    let show_result = matchs.get_flag("try");
    let selected_idx = matchs.get_one::<String>("select");
    //let mut actions: Vec<(String, String)> = Vec::new();
    let mut src_file_name = "".to_string();
    let mut tar_file_name = "".to_string();

    for (idx, seq) in results.clone().iter().enumerate() {
        let files = files.clone();
        let file_name = files.get(idx).unwrap();
        let mut output_seq:Vec<String> = Vec::new();
        output_seq.push(seq[0].to_string());
        output_seq.push("-> ".to_string());
        output_seq.push(seq[1].to_string());
        let mut ext = "".to_string();
        if let Some(selected_indies) = selected_idx {
            let indies = split_string_by_delimiters(selected_indies.as_str()).iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut new_name_seq:Vec<String> = Vec::new();
            let mut need_break = false;
            if let Some(seqs) = name_seqs.get_mut(idx){
                ext = seqs.iter().last().unwrap().to_string();
                for indie in indies {
                    if let Some(s) = seqs.get(indie as usize) {
                        new_name_seq.push(s.to_string());
                    }else{
                        need_break = true;
                        //println!("break 1 of idx:{} in {:?}", indie, seqs);
                    }
                }
            } else{
                need_break = true;
                //println!("break 2 of idx:{}", idx);
            }
            if need_break {
                println!("跳过文件:{}", file_name);
                continue;
            }
            //println!("new name seq:{:?}", new_name_seq);
            let new_name = format!("{}.{}", new_name_seq.join("_"), ext).to_string();
            //actions.push((file_name.to_string(), new_name.clone()));
            src_file_name = file_name.to_string();
            tar_file_name = new_name.clone();
            println!("pushed:{}", file_name);
            if show_result {
                output_seq.push(" = ".to_string());
                output_seq.push(new_name.to_string());
            }
        }
        if matchs.get_flag("do") {
            let tar = format!("./{}", tar_file_name);
            //println!("mv {} {}", src_file_name, tar);
            match fs::rename(src_file_name.as_str(), tar.as_str()) {
                Ok(()) => {
                    println!("{}->{} 成功", src_file_name, tar);
                }
                Err(e) => {
                    eprintln!("{}->{} 移动失败:{:?}", src_file_name, tar, e);
                }
            }
        } else {
            println!("{}", output_seq.join(" "));
        }
    }

}
