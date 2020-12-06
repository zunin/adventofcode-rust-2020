use itertools::Itertools;

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
}



pub fn get_number_of_valid_passwords(input: &[String]) -> usize

{
    tokenize_passport_data(input).into_iter().filter(validate_passport).count()
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

pub fn validate_passport(passport: &PassportData) -> bool {
    passport.byr.is_some()
        && passport.iyr.is_some()
        && passport.eyr.is_some()
        && passport.hgt.is_some()
        && passport.hcl.is_some()
        && passport.ecl.is_some()
        && passport.pid.is_some()
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
            println!("{:?}", data);

            data
        })
        .collect::<Vec<PassportData>>()
}
