use itertools::Itertools;
use std::ops::{Range, RangeInclusive};

#[cfg(test)]
mod tests {
    use super::*;
    use shared::get_input_txt;

    #[test]
    fn example_data_part_one() {
        let input = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ].to_vec()
            .into_iter()
            .map(String::from)
            .collect_vec();

        assert_eq!(get_number_of_valid_passwords(&input), 2);
    }

    #[test]
    fn part_one() {
        assert_eq!(
            get_number_of_valid_passwords(get_input_txt().as_slice()), 222);
    }

    #[test]
    fn example_valid_part_two() {
        assert_eq!(
            get_number_of_valid_passwords_strict_validation(vec![
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
                "hcl:#623a2f",
                "",
                "eyr:2029 ecl:blu cid:129 byr:1989",
                "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                "",
                "hcl:#888785",
                "hgt:164cm byr:2001 iyr:2015 cid:88",
                "pid:545766238 ecl:hzl",
                "eyr:2022",
                "",
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            ].to_vec()
                .into_iter()
                .map(String::from)
                .collect_vec().as_slice()), 4);
    }

    #[test]
    fn example_invalid_part_two() {
        assert_eq!(
            get_number_of_valid_passwords_strict_validation(vec![
                "eyr:1972 cid:100",
                "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                "",
                "iyr:2019",
                "hcl:#602927 eyr:1967 hgt:170cm",
                "ecl:grn pid:012533040 byr:1946",
                "",
                "hcl:dab227 iyr:2012",
                "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                "",
                "hgt:59cm ecl:zzz",
                "eyr:2038 hcl:74454a iyr:2023",
                "pid:3556412378 byr:2007"
            ].to_vec()
                .into_iter()
                .map(String::from)
                .collect_vec().as_slice()), 0);
    }

    #[test]
    fn part_two() {
        assert_eq!(
            get_number_of_valid_passwords_strict_validation(get_input_txt().as_slice()), 140);
    }
}



pub fn get_number_of_valid_passwords(input: &[String]) -> usize

{
    tokenize_passport_data(input).into_iter().filter(PassportData::validate_passport).count()
}

pub fn get_number_of_valid_passwords_strict_validation(input: &[String]) -> usize {
    tokenize_passport_data(input).into_iter().filter(PassportData::validate_passport_strict).count()
}

#[derive(Debug)]
pub struct PassportData {
    byr: Option<String>, //(Birth Year)
    iyr: Option<String>, //(Issue Year)
    eyr: Option<String>, //(Expiration Year)
    hgt: Option<String>, //(Height)
    hcl: Option<String>, //(Hair Color)
    ecl: Option<String>, //(Eye Color)
    pid: Option<String>, //(Passport ID)
    cid: Option<String>, //(Country ID)

}

impl PassportData {
    fn validate_year(string: &Option<String>, range: Range<usize>) -> bool {
        if let Some(year_string) = string {
            if let Ok(year) = year_string.parse::<usize>() {
                return range.contains(&year);
            }
        }
        false
    }

    fn validate_byr(&self) -> bool {
        PassportData::validate_year(&self.byr, 1920..2003)
    }

    fn validate_iyr(&self) -> bool {
        PassportData::validate_year(&self.iyr, 2010..2021)
    }

    fn validate_eyr(&self) -> bool {
        PassportData::validate_year(&self.eyr, 2020..2031)
    }

    fn validate_hgt(&self) -> bool {
        if let Some(height_string) = &self.hgt {
            let (number_string, format_string) = height_string
                .split_at(height_string.len() - 2);

            if let Ok(number) = number_string.parse::<usize>() {
                return match format_string {
                    "cm" => matches!(number, 150..=194),
                    "in" => matches!(number, 59..=76),
                    _ => false
                }
            }
        }
        false
    }

    fn validate_hcl(&self) -> bool {
        if let Some(hcl_string) = &self.hcl {
            let (pound_sign, hex_color) = hcl_string.split_at(1);
            if pound_sign != "#" {
                return false;
            }
            if hex_color.len() != 6 {
                return false;
            }

            return hex_color.chars().all(|character| {
                character.is_numeric() || matches!(character, 'a' | 'b' | 'c' | 'd' | 'e' | 'f')
            });

        }
        false
    }

    fn validate_ecl(&self) -> bool {
        if let Some(ecl_string) = &self.ecl {
            return matches!(
                ecl_string.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
        }
        false
    }

    fn validate_pid(&self) -> bool {
        if let Some(pid_string) = &self.pid {
            return pid_string.chars().all(char::is_numeric) && pid_string.len() == 9;
        }
        false
    }

    fn validate_passport_strict(&self) -> bool {

        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }

    fn validate_passport(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}



pub fn tokenize_passport_data(input :&[String]) -> Vec<PassportData>
{
    input.split(String::is_empty)
        .collect_vec()
        .into_iter()
        .map(|lines| {
            let joined_string: String = lines.join(" ");
            let components = joined_string.split_whitespace()
                .map(|component| {
                    let raw_tuple = component.split_at(component.find(':').unwrap());
                    (raw_tuple.0.to_string(), raw_tuple.1[1..].to_string())
                }).collect_vec();

            let mut data = PassportData {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None
            };

            for (key, value) in components {
                match key.as_str() {
                    "byr" => data.byr = Some(value),
                    "iyr" => data.iyr = Some(value),
                    "eyr" => data.eyr = Some(value),
                    "hgt" => data.hgt = Some(value),
                    "hcl" => data.hcl = Some(value),
                    "ecl" => data.ecl = Some(value),
                    "pid" => data.pid = Some(value),
                    "cid" => data.cid = Some(value),
                    _ => {}
                }
            }

            data
        })
        .collect::<Vec<PassportData>>()
}
