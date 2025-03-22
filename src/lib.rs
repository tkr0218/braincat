use std::error::Error;
use clap::{App,Arg};
type MyResult<T>=Result<T,Box<dyn Error>>;
use std::fs::File;
use std::io::{self,BufRead,BufReader};
static CAT: &[&str] = &["みゃお", "にゃん", "にゃー", "にゃにゃにゃ", "ごろごろ", "うにゃ", "（^・ω・^=）", "（=^・ω・^）"];
fn open(filename:&str)->MyResult<Box<dyn BufRead>>{
    match filename{
        "-"=>Ok(Box::new(BufReader::new(io::stdin()))),
        _=>Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
fn readnth(text:&str,x:usize)->MyResult<char>{
    Ok(text.chars().nth(x).unwrap())
}
pub fn run(config:Config)->MyResult<()>{
        let file_place =config.file;
        match open(&file_place){
            Ok(file)=>{
                let code=comp(file)?;
                let output=exec(code,&config.input)?;
                print!("{}",output);
            },
            Err(err)=>eprintln!("Failed to open {}: {}",file_place,err),
        }
    Ok(())
}
pub fn exec(code:Vec<i32>,input:&str)->MyResult<String>{
    let mut output=String::new();
    let mut tape=[0;30000];
    let mut dp=0;
    let mut ip=0;
    let mut x=0;
    while ip<code.len(){
        match code[ip]{
            0=>dp+=1,
            1=>dp-=1,
            2=>tape[dp]+=1,
            3=>tape[dp]-=1,
            4=>output+=&(tape[dp] as u8 as char).to_string(),
            5=>{match readnth(&input,x){Ok(co)=>{tape[dp]=co as i32;x+=1;},Err(_)=>return Err("few input".into()),}},
            6=>if tape[dp]==0{
                let mut loop_count=1;
                while loop_count>0{
                    ip+=1;
                    if code[ip]==6{
                        loop_count+=1;
                    }else if code[ip]==7{
                        loop_count-=1;
                    }
                }
            },
            7=>if tape[dp]!=0{
                let mut loop_count=1;
                while loop_count>0{
                    ip-=1;
                    if code[ip]==6{
                        loop_count-=1;
                    }else if code[ip]==7{
                        loop_count+=1;
                    }
                }
            },
            _=>return Err("Invalid code".into()),
        }
        ip+=1;
    }
    Ok(output)
}
pub fn comp(file: Box<dyn BufRead>) -> MyResult<Vec<i32>> {
    let reader = file;
    let mut code: Vec<i32> = Vec::new();
    let mut cd=String::new();
    let mut count=0;
    for line in reader.lines() {
        let line = line?;
        for c in line.chars() {
            if c=='\n'{
                continue;
            }
            cd += &c.to_string();
            if cd == CAT[0] {
                code.push(0);
                cd.clear();
                count = 0;
            } else if cd == CAT[1] {
                code.push(1);
                cd.clear();
                count = 0;
            } else if cd == CAT[2] {
                code.push(2);
                cd.clear();
                count = 0;
            } else if cd == CAT[3] {
                code.push(3);
                cd.clear();
                count = 0;
            } else if cd == CAT[4] {
                code.push(4);
                cd.clear();
                count = 0;
            } else if cd == CAT[5] {
                code.push(5);
                cd.clear();
                count = 0;
            } else if cd == CAT[6] {
                code.push(6);
                cd.clear();
                count = 0;
            } else if cd == CAT[7] {
                code.push(7);
                cd.clear();
                count = 0;
            } else {
                count += 1;
            }
            if count > 10 {
                print!("{}",cd);
                return Err("Invalid code".into());
            }
        }
    }
    Ok(code)
}

pub fn get_args()->MyResult<Config>{
    let matches=App::new("brainr")
    .version("0.1.0")
    .author("neko")
    .about("Rust cat brainfuck interpreter")
    .arg(
        Arg::with_name("file")
        .value_name("FILE")
        .help("Input file")
        .default_value("-")
        .required(true)
        .multiple(false),
    ).arg(
        Arg::with_name("input")
        .value_name("TEXT")
        .help("Input text")
        .default_value("")
        .required(false)
        .multiple(false),
    )
    .get_matches();
    Ok(Config{
        file:matches.value_of("file").unwrap().to_string(),
        input:matches.value_of("input").unwrap().to_string(),
    })
}
#[derive(Debug)]
pub struct Config{
    file:String,
    input:String,
}