// use std::collections::HashMap;
use std::{
    // collections::BTreeMap,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use std::time::Instant;

fn main() {
    let t1 = Instant::now();
    let mut city_stats: HashMap<String, (usize, f32, f32, f32)> = HashMap::new();
    // let mut cities: Vec<String> = vec![];
    // let mut city_stats: Vec<(usize, f32, f32, f32)> = vec![];
    let fp = File::open("/Users/aaronfinke/1brc/measurements.txt").unwrap();
    let reader = BufReader::<File>::new(fp);

    for line in reader.lines() {
        if let Ok(f) = line {
            // if f.starts_with('#') {
            //     continue;
            // };
            // if let Some((c, temp)) = f.split_once(';') {
            //     let city = String::from(c);
            //     let temps:f32 = temp.parse().unwrap();
            //     let _ = *city_stats.entry(city.to_string())
            //     .and_modify(|stats| {
            //         stats.0 += 1;
            //         stats.1 += temps;
            //         if temps < stats.2 {stats.2 = temps};
            //         if temps > stats.3 {stats.3 = temps};
            //     })
            //     .or_insert((1,temps,temps,temps));
                // if let Some(i) = cities.iter().position(|r| r == &city) {
                //     city_stats[i].0 += 1;
                //     city_stats[i].1 += temps;
                //     if temps < city_stats[i].2 {city_stats[i].2 = temps};
                //     if temps > city_stats[i].3 {city_stats[i].3 = temps};
                // } else {
                //     cities.push(city.to_string());
                //     city_stats.push((1,temps,temps,temps));
                // }
                
            // }
        }
    }
    // for (city,values) in city_stats.into_iter() {
    //     let max = values.3;
    //     let min = values.2;
    //     let average = values.1 / values.0 as f32;
    //     println!("{}: {}/{:.1}/{}",city,min,average,max);
    // }

    println!("time: {:?}", t1.elapsed());
}
