use std::collections::HashMap;

pub fn get_filename(filepath: &str, file_format: &str) -> String {
    let filename: Vec<&str> = filepath.split('.').collect();
    let after_filename = format!("{}.{}", filename[0], file_format);

    after_filename
}

fn parse_options(options: &HashMap<String, usize>) -> String {
    let mut option_vec: Vec<String> = Vec::new();

    if let Some(&val) = options.get("qv") {
        let opt = format!(" -q:v {}", val);
        option_vec.push(opt)
    }

    if let Some(&val) = options.get("ab") {
        let opt = format!(" -ab {}", val);
        option_vec.push(opt)
    }

    let mut option_arg = String::from("");

    for i in option_vec.iter() {
        option_arg += i
    }

    option_arg
}

pub fn ffmpeg_cmd(
    filepath: &str,
    file_format: &str,
    options: Option<&HashMap<String, usize>>,
) -> String {
    let mut option_arg = String::new();

    if let Some(val) = options {
        option_arg = parse_options(val);
    }

    let after_filename = get_filename(filepath, file_format);
    let cmd = format!("ffmpeg -y -i {}{} {}", filepath, option_arg, after_filename);

    cmd
}

#[cfg(test)]
mod tests {
    use super::parse_options;
    use maplit::hashmap;
    use rstest::rstest;
    use std::collections::HashMap;

    #[rstest(options, result, case(&hashmap! {"qv".to_string() => 1}, " -q:v 1"), case(&hashmap! {"qv".to_string() => 12}, " -q:v 12"), case(&hashmap! {"qv".to_string() => 12,"ab".to_string() => 192}, " -q:v 12 -ab 192"))]
    fn test_parse_options(options: &HashMap<String, usize>, result: &str) {
        assert_eq!(parse_options(options), result)
    }
}
