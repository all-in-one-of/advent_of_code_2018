macro_rules! day {
    ($name:tt, $url:tt, $part1:ident, $part2:ident) => {
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
    (unimplemented) => {
        None
    };
    ($callback:ident) => {
        Some((|input| $callback(input).map(|x| x.to_string())) as (fn(String) -> Result<String>))
    };
}
#[cfg(test)]
macro_rules! assert_result {
    ($fn:ident, $expected:expr, $input:tt) => {
        assert_eq!($expected, $fn($input.to_owned()).expect("function should run without error"));
    };
}
#[cfg(test)]
macro_rules! assert_results {
    ($fn:ident, $($input:tt => $expected:expr),+$(,)*) => {
        $(
            assert_result!($fn, $expected, $input);
        )+;
    };
}

use crate::{Error, Result};
use reqwest::{Client, StatusCode};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct Framework {
    days: BTreeMap<&'static str, Day>,
    token: Option<String>,
    input_cache: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Day {
    name: &'static str,
    url: &'static str,
    part1: Option<fn(String) -> Result<String>>,
    part2: Option<fn(String) -> Result<String>>,
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
        }
    }

    pub fn register_day(
        &mut self,
        name: &'static str,
        url: &'static str,
        part1: Option<fn(String) -> Result<String>>,
        part2: Option<fn(String) -> Result<String>>,
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

    fn fetch_input(&mut self, client: &Client, url: &'static str) -> Result<String> {
        if let Some(v) = self.input_cache.get(url) {
            return Ok(v.clone());
        }
        let token = self.token.as_ref().ok_or(Error::MissingSessionToken)?;

        let mut response = client.get(url)
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
        
        self.input_cache.insert(url.to_owned(), result.clone());

        let serialized = ::bincode::serialize(&self.input_cache)?;
        ::std::fs::write("cache.dat", serialized)?;

        Ok(result)
    }

    pub fn execute(&mut self, client: &Client, day: &str) -> Result<()> {
        let day = self.days.get(day).ok_or_else(|| Error::DayDoesNotExist(day.to_owned()))?.clone();

        let input = self.fetch_input(client, day.url)?;
        if let Some(part1) = day.part1 {
            println!("\nRunning {}, part 1", day.name);
            println!("Result:\n{}", part1(input.clone())?);
        }
        if let Some(part2) = day.part2 {
            println!("\nRunning {}, part 2", day.name);
            println!("Result:\n{}", part2(input)?);
        }

        Ok(())
    }
}
