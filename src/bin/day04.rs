use std::collections::HashMap;

use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
lazy_static! {
    static ref PATTERNS: HashMap<&'static str, Regex> = {
        let mut map = HashMap::new();
        map.insert("byr", Regex::new(r"^\d{4}$").unwrap());
        map.insert("iyr", Regex::new(r"^\d{4}$").unwrap());
        map.insert("eyr", Regex::new(r"^\d{4}$").unwrap());
        map.insert("hgt", Regex::new(r"^(\d+)(cm|in)$").unwrap());
        map.insert("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap());
        map.insert("ecl", Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
        map.insert("pid", Regex::new(r"^\d{9}$").unwrap());
        map
    };
}

#[derive(Debug, Default)]
pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn add_field(&mut self, name: &str, value: &str) {
        self.fields.insert(name.to_string(), value.to_string());
    }

    pub fn has_required_fields(&self) -> bool {
        // make sure all required fields are present
        REQUIRED_FIELDS.iter().all(|f| self.fields.contains_key(*f))
            && (self.fields.len() == 7
                || (self.fields.len() == 8 && self.fields.contains_key("cid")))
    }

    pub fn are_fields_valid(&self) -> bool {
        fn check_year(input: &str, min: u16, max: u16) -> bool {
            if let Ok(year) = input.parse::<u16>() {
                return year >= min && year <= max;
            } else {
                return false;
            }
        }

        for req_field in &REQUIRED_FIELDS {
            if let Some(value) = self.fields.get(*req_field) {
                let pattern = PATTERNS.get(*req_field).unwrap();
                let valid = match *req_field {
                    "byr" => check_year(value, 1920, 2002),
                    "iyr" => check_year(value, 2010, 2020),
                    "eyr" => check_year(value, 2020, 2030),
                    "hgt" => {
                        if let Some(captures) = pattern.captures(value) {
                            let v = captures
                                .get(1)
                                .map(|v| v.as_str())
                                .and_then(|v| v.parse::<u16>().ok());
                            let unit = captures.get(2).map(|v| v.as_str());
                            v.zip(unit)
                                .map(|(v, unit)| match unit {
                                    "cm" => v >= 150 && v <= 193,
                                    "in" => v >= 59 && v <= 76,
                                    _ => false,
                                })
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    }
                    "hcl" => pattern.is_match(value),
                    "ecl" => pattern.is_match(value),
                    "pid" => pattern.is_match(value),
                    _ => false,
                };

                if !valid {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        self.has_required_fields() && self.are_fields_valid()
    }
}

pub fn parse_passports(input: &str) -> Result<Vec<Passport>> {
    let mut passports = Vec::new();
    let mut current_passport = None;
    for line in input.lines() {
        if line.is_empty() {
            passports.push(current_passport.take().unwrap());
            continue;
        }
        let p = current_passport.get_or_insert(Passport::default());
        for entry in line.split(' ') {
            let mut fields = entry.split(':');
            let name = fields
                .next()
                .ok_or(format_err!("invalid entry! entry={}", entry))?;
            let value = fields
                .next()
                .ok_or(format_err!("invalid entry! entry={}", entry))?;
            p.add_field(name, value);
        }
    }

    if let Some(p) = current_passport {
        passports.push(p);
    }

    Ok(passports)
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let passports = parse_passports(&input)?;

    let valid = passports.iter().filter(|p| p.has_required_fields()).count();
    println!("part 1: {}", valid);

    let valid = passports
        .iter()
        .filter(|p| p.has_required_fields() && p.are_fields_valid())
        .count();
    println!("part 2: {}", valid);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_passports() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let passports = parse_passports(input).unwrap();

        assert_eq!(4, passports.len());
        for p in &passports {
            assert_eq!(false, p.is_valid());
        }
    }

    #[test]
    fn test_valid_passports() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports = parse_passports(input).unwrap();

        assert_eq!(4, passports.len());
        for p in &passports {
            assert_eq!(true, p.is_valid(), "passport {:?} should be valid", p);
        }
    }
}
