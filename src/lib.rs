use std::{env, error::Error, fs};

pub fn config() -> Result<(), Box<dyn Error>> {
    let path = if cfg!(test) { ".env.test" } else { ".env" };
    let result: String = fs::read_to_string(path)?.parse()?;

    for line in result.split('\n') {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line.starts_with('#') {
            continue;
        }

        let splitted = trimmed_line.split('=').collect::<Vec<_>>();
        let key = splitted[0];
        let mut value = splitted[1..].join("");
        if value.starts_with('\"') && value.ends_with('\"') {
            value = value
                .trim_start_matches('\"')
                .trim_end_matches('\"')
                .to_string();
        }

        env::set_var(key, value);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        config().unwrap();

        assert_eq!(env::var("SAMPLE_KEY1").unwrap(), "SAMPLE_VALUE1");
        assert_eq!(env::var("SAMPLE_KEY2").unwrap(), "SAMPLE_VALUE2");
    }
}
