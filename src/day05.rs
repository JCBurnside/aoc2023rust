use std::str::FromStr;
use itertools::Itertools;

struct Data {
    seeds : Vec<usize>,
    seed2soil : Vec<(usize,usize,usize)>,
    soil2fert : Vec<(usize,usize,usize)>,
    fert2wata : Vec<(usize,usize,usize)>,
    wata2light : Vec<(usize,usize,usize)>,
    light2temp : Vec<(usize,usize,usize)>,
    temp2humid : Vec<(usize,usize,usize)>,
    humid2loc : Vec<(usize,usize,usize)>,
}

impl Data {
    fn get_seed_loc(&self,seed:usize) -> usize {
        let soil = self.seed2soil.iter().find_map(|(soil,start,range)| {
            (start <= &seed &&  start + range > seed).then(|| soil + (seed - start))
        }).unwrap_or(seed);
        
        let fert = self.soil2fert.iter().find_map(|(fert,start,range)| {
            (start <= &soil &&  start + range > soil).then(|| fert + (soil - start))
        }).unwrap_or(soil);
        
        let wata = self.fert2wata.iter().find_map(|(wata,start,range)| {
            (start <= &fert &&  start + range > fert).then(|| wata + (fert - start))
        }).unwrap_or(fert);
        
        let light = self.wata2light.iter().find_map(|(light,start,range)| {
            (start <= &wata &&  start + range > wata).then(|| light + (wata - start))
        }).unwrap_or(wata);
        
        let temp = self.light2temp.iter().find_map(|(temp,start,range)| {
            (start <= &light &&  start + range > light).then(|| temp + (light - start))
        }).unwrap_or(light);
        
        let humid = self.temp2humid.iter().find_map(|(humid,start,range)| {
            (start <= &temp &&  start + range > temp).then(|| humid + (temp - start))
        }).unwrap_or(temp);
        
        let loc = self.humid2loc.iter().find_map(|(loc,start,range)| {
            (start <= &humid &&  start + range > humid).then(|| loc + (humid - start))
        }).unwrap_or(humid);
        
        loc
    }
}
const INPUT : &'static str = include_str!("../data/day05.txt");
fn parse(input:&str) -> Data {
    let mut lines = input.trim().lines();
    let seeds = lines.next().unwrap();
    let seeds = seeds[7..].split_whitespace().map(usize::from_str).map(Result::unwrap).collect();
    let _ = lines.next();//should be a blank line
    let _ = lines.next();//should the label
    let seed2soil  = { 
        let mut seed2soil = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            seed2soil.push(line);
        }
        seed2soil
    };

    let _ = lines.next();//should be the label
    let soil2fert = 
    {
        let mut soil2fert = Vec::with_capacity(seed2soil.capacity());
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            soil2fert.push(line);
        }
        soil2fert
    };
    let _ = lines.next();//should be the label
    let fert2wata = 
    {
        let mut fert2wata = Vec::with_capacity(seed2soil.capacity());
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            fert2wata.push(line);
        }
        fert2wata
    };
    let _ = lines.next();//should be the label
    let wata2light = 
    {
        let mut wata2light = Vec::with_capacity(seed2soil.capacity());
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            wata2light.push(line);
        }
        wata2light
    };
    let _ = lines.next();//should be the label
    let light2temp = 
    {
        let mut light2temp = Vec::with_capacity(seed2soil.capacity());
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            light2temp.push(line);
        }
        light2temp
    };
    let _ = lines.next();//should be the label
    let temp2humid = 
    {
        let mut temp2humid = Vec::with_capacity(seed2soil.capacity());
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            temp2humid.push(line);
        }
        temp2humid
    };
    let _ = lines.next();//should be the label
    let humid2loc = 
    {
        let mut humid2loc = Vec::with_capacity(seed2soil.capacity());
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let line = line.split_whitespace().map(usize::from_str).map(Result::unwrap).collect_tuple().unwrap();
            humid2loc.push(line);
        }
        humid2loc
    };
    Data {
        seeds,
        seed2soil,
        soil2fert,
        fert2wata,
        wata2light,
        light2temp,
        temp2humid,
        humid2loc
    }
}

fn part1_impl(data:Data) -> usize {
    data.seeds
    .iter()
    .map(|seed| data.get_seed_loc(*seed))
    .min().unwrap()
}

pub fn part1() {
    let data = parse(INPUT);
    println!("lowest location is {}",part1_impl(data));
}

pub fn part2() {
    println!("not yet implemented")
}

#[cfg(test)]
mod tests {
    const SAMPLE : &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn part1() {
        let data = super::parse(SAMPLE);
        assert_eq!(super::part1_impl(data),35)
    }
}