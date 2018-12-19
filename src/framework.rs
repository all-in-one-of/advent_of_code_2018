macro_rules! day {
    ($name:tt, $url:tt, $part1:tt, $part2:tt) => {
        #[allow(unused_imports)]
        use crate::{Error, Result};
        pub(crate) fn register_day(fw: &mut crate::framework::Framework) {
            fw.register_day(
                stringify!($name),
                $url,
                day_callback!($part1),
                day_callback!($part2),
            );
        }
    };
}
macro_rules! day_callback {
    (!) => {
        None
    };
    ($callback:ident) => {
        Some((|input| $callback(input).map(|x| x.to_string())) as (fn(&str) -> Result<String>))
    };
}
#[allow(unused_macros)]
#[cfg(test)]
macro_rules! assert_results {
    ($fn:ident, $($input:tt => $expected:expr),+$(,)*) => {
        $(
            assert_eq!(
                $fn($input).expect("function should run without error"),
                $expected
            );
        )+;
    };
}

use crate::{Error, Result};
use reqwest::{Client, StatusCode};
use std::collections::{BTreeMap, HashMap};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Framework {
    days: BTreeMap<&'static str, Day>,
    token: Option<String>,
    input_cache: HashMap<String, String>,
    no_fetch_before: Option<Instant>,
}

#[derive(Clone)]
struct Day {
    name: &'static str,
    url: &'static str,
    part1: Option<fn(&str) -> Result<String>>,
    part2: Option<fn(&str) -> Result<String>>,
}

impl Framework {
    pub fn new() -> Framework {
        use std::fs::{read, read_to_string};
        let token = read_to_string("token.txt").ok();
        let input_cache = read("cache.dat")
            .ok()
            .and_then(|x| ::bincode::deserialize::<HashMap<String, String>>(&x[..]).ok())
            .unwrap_or(HashMap::new());
        Framework {
            days: BTreeMap::new(),
            token,
            input_cache,
            no_fetch_before: None,
        }
    }

    pub fn register_day(
        &mut self,
        name: &'static str,
        url: &'static str,
        part1: Option<fn(&str) -> Result<String>>,
        part2: Option<fn(&str) -> Result<String>>,
    ) -> bool {
        if self.days.contains_key(&name) {
            return false;
        }

        let day = Day {
            name,
            url,
            part1,
            part2,
        };
        self.days.insert(name, day);

        true
    }

    fn cache_input(&mut self, client: &Client, url: &'static str) -> Result<()> {
        if let Some(no_fetch_before) = self.no_fetch_before {
            let now = Instant::now();
            if now < no_fetch_before {
                std::thread::sleep(no_fetch_before - now);
            }
        }

        if self.input_cache.get(url).is_some() {
            return Ok(());
        }
        let token = self.token.as_ref().ok_or(Error::MissingSessionToken)?;

        let mut response = client
            .get(url)
            .header("cookie", format!("session={}", token))
            .send()?;

        if response.status() != StatusCode::OK {
            return Err(Error::InvalidSessionToken(response.status()));
        }

        let mut result = response.text()?;

        // Strip trailing newline characters
        while let Some(last) = result.pop() {
            if last == '\r' || last == '\n' {
                continue;
            }
            result.push(last);
            break;
        }

        self.input_cache.insert(url.to_owned(), result);

        let serialized = ::bincode::serialize(&self.input_cache)?;
        ::std::fs::write("cache.dat", serialized)?;

        self.no_fetch_before = Some(Instant::now() + Duration::from_secs(5));

        Ok(())
    }

    pub fn execute(&mut self, client: &Client, day: &str) -> Result<()> {
        let day = self
            .days
            .get(day)
            .ok_or_else(|| Error::DayDoesNotExist(day.to_owned()))?
            .clone();

        self.cache_input(client, day.url)?;
        let input = self.input_cache.get(day.url).unwrap();
        if let Some(part1) = day.part1 {
            println!("\nRunning {}, part 1", day.name);
            println!("Result:\n{}", part1(input)?);
        }
        if let Some(part2) = day.part2 {
            println!("\nRunning {}, part 2", day.name);
            println!("Result:\n{}", part2(input)?);
        }

        Ok(())
    }
}
