use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

pub fn read(day: usize) -> Result<Vec<String>, io::Error> {
    let path = format!("inputs/day{:02}.txt", day);
    let file = File::open(path)?;
    let file = BufReader::new(file);
    file.lines().collect()
}

pub fn pack(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    let mut pack = Vec::new();
    for l in lines {
        if !l.is_empty() {
            pack.push(l);
            continue;
        }

        result.push(pack);
        pack = Vec::new();
    }

    if !pack.is_empty() {
        result.push(pack);
    }

    result
}

pub fn convert<T: FromStr>(lines: Vec<String>) -> Result<Vec<T>, <T as FromStr>::Err> {
    lines.iter().map(|l| l.parse()).collect()
}

pub fn convert_pack<T: FromStr>(packs: Vec<Vec<String>>) -> Result<Vec<Vec<T>>, <T as FromStr>::Err> {
    packs.into_iter().map(convert::<T>).collect()
}

#[macro_export] macro_rules! impl_from_str {
    ($impl_type:ty) => {
       impl std::str::FromStr for $impl_type {
           type Err = nom::error::Error<String>;

           fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
               let result = <$impl_type>::parse(s);
               let finished = nom::Finish::finish(result);
               match finished {
                   Ok((_, v)) => Ok(v),
                   Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                       input: input.to_string(),
                       code,
                   }),
               }
           }
       }
    }
}
