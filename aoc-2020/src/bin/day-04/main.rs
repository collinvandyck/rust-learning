use aoc_2020::prelude::*;

#[test]
fn test_invalid() {
    let passports = get_passports("invalid.txt").unwrap();
    for passport in passports {
        assert!(
            !passport.valid(Validation::Strict),
            "the passport {passport:?} should have been invalid"
        )
    }
}

#[test]
fn test_valid() {
    let passports = get_passports("valid.txt").unwrap();
    for passport in passports {
        assert!(
            passport.valid(Validation::Strict),
            "the passport {passport:?} should have been valid"
        )
    }
}

fn main() -> Result<()> {
    let p1 = (
        num_valid_passports("example.txt", Validation::Loose)?,
        num_valid_passports("input.txt", Validation::Loose)?,
    );
    println!("p1: {p1:?}");
    let p2 = (
        num_valid_passports("example.txt", Validation::Strict)?,
        num_valid_passports("input.txt", Validation::Strict)?,
    );
    println!("p2: {p2:?}");
    Ok(())
}

fn num_valid_passports(p: impl AsRef<Path>, validation: Validation) -> Result<usize> {
    Ok(get_passports(p)?
        .into_iter()
        .filter(|p| p.valid(validation))
        .count())
}

fn get_passports(p: impl AsRef<Path>) -> Result<Vec<Passport>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let mut lines = file_to_lines(p)?.into_iter();
    let mut passports = vec![];
    loop {
        let line = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if line.is_empty() {
            break;
        }
        let map = line
            .split(" ")
            .map(|s| s.splitn(2, ":").collect::<Vec<_>>())
            .map(|p| {
                let field = Field::from(p[0]).unwrap();
                let value = p[1].to_string();
                (field, value)
            })
            .collect::<HashMap<_, _>>();
        let passport = Passport::new(map);
        passports.push(passport);
    }
    Ok(passports)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Validation {
    Loose,
    Strict,
}

#[derive(Debug, Clone)]
struct Passport {
    values: HashMap<Field, String>,
}

impl Passport {
    fn new(values: HashMap<Field, String>) -> Self {
        Self { values }
    }

    fn valid(&self, validation: Validation) -> bool {
        let required = FIELDS.iter().filter(|f| !f.optional);
        for field in required {
            let Some(val) = self.values.get(field) else {
                return false;
            };
            if validation == Validation::Strict {
                if !field.validator.validate(val) {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Validator {
    Year {
        min: i32,
        max: i32,
    },
    Height {
        cm_min: i32,
        cm_max: i32,
        in_min: i32,
        in_max: i32,
    },
    HairColor,
    EyeColor,
    PassportID,
    None,
}

impl Validator {
    fn validate(&self, val: &str) -> bool {
        match self {
            Validator::Year { min, max } => {
                let chs: Vec<_> = val.chars().collect();
                if chs.len() != 4 {
                    return false;
                }
                let Ok(date) = val.parse::<i32>() else {
                    return false;
                };
                date >= *min && date <= *max
            }
            Validator::Height {
                cm_min,
                cm_max,
                in_min,
                in_max,
            } => {
                if !val.ends_with("in") && !val.ends_with("cm") {
                    return false;
                }
                let Ok(amt) = &val[0..val.len() - 2].parse::<i32>() else {
                    return false;
                };
                let unit = &val[val.len() - 2..];
                match unit {
                    "in" => amt >= in_min && amt <= in_max,
                    "cm" => amt >= cm_min && amt <= cm_max,
                    _ => unreachable!(),
                }
            }
            Validator::HairColor => {
                if !val.starts_with('#') {
                    return false;
                }
                let rest = &val[1..];
                if rest.len() != 6 {
                    return false;
                }
                rest.chars()
                    .all(|c| c.is_ascii_hexdigit() && (c.is_digit(10) || c.is_lowercase()))
            }
            Validator::EyeColor => {
                static COLORS: Lazy<Vec<&'static str>> =
                    Lazy::new(|| vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
                COLORS.contains(&val)
            }
            Validator::PassportID => {
                if val.len() != 9 {
                    return false;
                }
                val.chars().all(|c| c.is_digit(10))
            }
            Validator::None => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Field {
    name: &'static str,
    optional: bool,
    validator: Validator,
}

impl Field {
    fn from(name: impl AsRef<str>) -> Option<Field> {
        let name = name.as_ref();
        FIELDS.iter().find(|f| f.name == name).copied()
    }
}

static FIELDS: Lazy<Vec<Field>> = Lazy::new(|| {
    vec![
        Field {
            name: "byr",
            optional: false,
            validator: Validator::Year {
                min: 1920,
                max: 2002,
            },
        },
        Field {
            name: "iyr",
            optional: false,
            validator: Validator::Year {
                min: 2010,
                max: 2020,
            },
        },
        Field {
            name: "eyr",
            optional: false,
            validator: Validator::Year {
                min: 2020,
                max: 2030,
            },
        },
        Field {
            name: "hgt",
            optional: false,
            validator: Validator::Height {
                cm_min: 150,
                cm_max: 193,
                in_min: 59,
                in_max: 76,
            },
        },
        Field {
            name: "hcl",
            optional: false,
            validator: Validator::HairColor,
        },
        Field {
            name: "ecl",
            optional: false,
            validator: Validator::EyeColor,
        },
        Field {
            name: "pid",
            optional: false,
            validator: Validator::PassportID,
        },
        Field {
            name: "cid",
            optional: true,
            validator: Validator::None,
        },
    ]
});
