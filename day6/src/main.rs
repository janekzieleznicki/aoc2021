use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct LanternFish {
    timer: i16,
}

impl LanternFish {
    pub fn tick(&mut self) -> Option<LanternFish> {
        self.timer -= 1;
        if self.timer < 0 {
            self.timer = 6;
            return Some(LanternFish { timer: 8 });
        }
        None
    }
}

impl FromStr for LanternFish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(timer) => Ok(LanternFish { timer: timer }),
            Err(_) => Err(())
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
struct LanternFishSchool(Vec<LanternFish>);

impl LanternFishSchool {
    pub fn tick(&mut self) -> &mut Self {
        let new_fishes: Vec<LanternFish> = self.0.iter_mut()
            .filter_map(|fish| fish.tick())
            .collect();
        self.0.extend(new_fishes);
        // println!("Fishes: {:?}", self.0);
        self
    }
    pub fn count(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for LanternFishSchool {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LanternFishSchool(
            s.split(',').into_iter()
                .filter_map(|fish_str| fish_str.parse().ok())
                .collect()
        ))
    }
}

fn fish_at_day(string: &str, days: usize) -> usize {
    let mut immature_fish = VecDeque::from([0, 0]);
    let mut new_fish_at_day: HashMap<usize, u128> = HashMap::from(
        [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    string.split(',')
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .for_each(|int: usize| {
            let fish = new_fish_at_day.entry(int).or_insert(0);
            *fish += 1;
        });
    let mut day = 0;
    for _ in 0..=days {
        day += 1;
        if day == 7 { day = 0; }
        let fishes_to_spawn = *new_fish_at_day.get(&day).unwrap();

        immature_fish.push_back(fishes_to_spawn);
        if let Some(fishes_at_today) = new_fish_at_day.get_mut(&day) {
            *fishes_at_today = *fishes_at_today + immature_fish.pop_front().unwrap();
        }
        assert_eq!(immature_fish.len(), 2);

        println!("{:#?}", new_fish_at_day);
    }
    new_fish_at_day.iter().fold(0, |acc, (_, count)| acc + *count as usize)
}

#[cfg(test)]
mod tests {
    use crate::{fish_at_day, LanternFishSchool};

    #[test]
    fn with_test_data() {
        fish_at_day("3,4,3,1,2", 80);
        let mut fishes: LanternFishSchool = "3,4,3,1,2".parse().unwrap();
        fishes.tick();
        assert_eq!(fishes, "2,3,2,0,1".parse().unwrap());
        fishes.tick();
        assert_eq!(fishes, "1,2,1,6,0,8".parse().unwrap());
        fishes.tick();
        assert_eq!(fishes, "0,1,0,5,6,7,8".parse().unwrap());
        fishes.tick();
        assert_eq!(fishes, "6,0,6,4,5,6,7,8,8".parse().unwrap());
    }

    #[test]
    fn test_data() {
        for i in 0..=18 {
            let mut fishes: LanternFishSchool = "3,4,3,1,2".parse().unwrap();
            for _ in 0..i { fishes.tick(); }
            println!("{:?}", fishes);
            assert_eq!(fish_at_day("3,4,3,1,2", i), fishes.count());
        }
        let mut fishes: LanternFishSchool = "3,4,3,1,2".parse().unwrap();
        for _ in 1..=18 { fishes.tick(); }
        assert_eq!(fishes.count(), 26);
        assert_eq!(fish_at_day("3,4,3,1,2", 18), 26);
        for _ in 19..=80 { fishes.tick(); }
        assert_eq!(fishes.count(), 5934);
        assert_eq!(fish_at_day("3,4,3,1,2", 80), 5934);
    }
}

fn main() {
    let str = fs::read_to_string("day6/input_data.dat").unwrap();
    let mut fishes: LanternFishSchool = str.parse().unwrap();
    for i in 1..=80 {
        fishes.tick();
        println!("Day {}: {} fishes", i, fishes.count())
    }
    println!("Day 256: {} fishes", fish_at_day(str.as_str(), 256));
}
