use std::io::BufRead;

use regex::Regex;

/// Parse columns from the given `input` & `delimiter`, passing the resulting
/// column into the provided closure. The closure can be used to process
/// the column value:
///
/// ```
/// // Accumulate the values
/// use pc_rs::parse_columns;
///
/// # fn main() -> Result<(), String> {
/// let input = "1 2 3 4\na b c d";
/// let input = Box::new(std::io::BufReader::new(input.as_bytes()));
/// let mut results = Vec::new();
/// parse_columns(input, 1, " ", |col| results.push(col.to_owned())).unwrap();
/// assert_eq!(&results, &["1", "a"]);
/// # Ok(())
/// # }
/// ```
///
/// ```
/// // Print values
/// use pc_rs::parse_columns;
///
/// # fn main() -> Result<(), String> {
/// let input = "1 2 3 4\na b c d";
/// let input = Box::new(std::io::BufReader::new(input.as_bytes()));
/// parse_columns(input, 1, " ", |col| println!("{col}")).unwrap();
/// # Ok(())
/// # }
/// ```
pub fn parse_columns<'a, 'b, F>(
    input: Box<dyn BufRead + 'a>,
    column: usize,
    delimiter: &'b str,
    mut f: F,
) -> anyhow::Result<()>
where
    F: FnMut(&str),
{
    let delimiter_re = Regex::new(&format!("{delimiter}+"))?;
    for line in input.lines() {
        let line = line?;
        let col = parse_column(&line, column, &delimiter_re)?;
        f(col);
    }
    Ok(())
}

fn parse_column<'a>(line: &'a str, column: usize, delimiter_re: &Regex) -> anyhow::Result<&'a str> {
    match column {
        0 => Ok(line),
        c => {
            let col = delimiter_re
                .split(line)
                .nth(c - 1)
                .ok_or_else(|| anyhow::anyhow!("Column overflow"))?;
            Ok(col)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_col {
        ($input:expr, $column:expr, $delim:expr) => {{
            let input = $input;
            let mut results = Vec::new();
            parse_columns(
                Box::new(std::io::BufReader::new(input.as_bytes())),
                $column,
                $delim,
                |col| results.push(col.to_owned()),
            )
            .unwrap();
            results
        }};
    }

    #[test]
    fn test_parse_columns() {
        assert_eq!(parse_col!("1 2 3 4\na b c d", 1, " "), &["1", "a"]);
        assert_eq!(parse_col!("1 2 3 4\na b c d", 2, " "), &["2", "b"]);

        assert_eq!(parse_col!("1,2,3,4\na,b,c,d", 3, ","), &["3", "c"]);
    }
}
