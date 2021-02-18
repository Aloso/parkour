use palex::{Input, StringInput};

#[derive(Debug)]
struct Subcommand {
    r: u8,
    g: u8,
    b: u8,
}

pub fn main() -> Result<(), String> {
    let mut input = StringInput::new(std::env::args());

    // Ignore the first argument
    input.bump_argument().unwrap();

    // The values to be parsed
    let mut verbose = false;
    let mut output: Option<String> = None;
    let mut subcommand: Option<Subcommand> = None;

    // consume tokens until the input is empty
    while !input.is_empty() {
        // All methods that start with `eat` work the same: They check if the current
        // token matches the argument; if the check succeeds, the token is
        // consumed and returned. This is comparable to Iterator::next_if().

        // When the -h/--help flag is encountered, the help is printed and the program
        // is exited
        if input.eat_one_dash("h").is_some() || input.eat_two_dashes("help").is_some() {
            print_help();
            return Ok(());
        }

        // When the -v/--verbose flag is encountered, `verbose` is set to true
        if input.eat_one_dash("v").is_some() || input.eat_two_dashes("verbose").is_some()
        {
            // This produces an error if the flag is encountered for a second time
            if verbose {
                return Err("The `--verbose` flag was given multiple times".into());
            }
            verbose = true;
            continue;
        }

        // The -o/--out argument expects a string value.
        // This value can be given after a equals sign (e.g. `--out=foo`) or a space
        // (e.g. `--out foo`)
        if input.eat_one_dash("o").is_some() || input.eat_two_dashes("out").is_some() {
            // Here we parse the value, which might start with a dash
            if let Some(path) = input.value_allows_leading_dashes() {
                // Assign the string to the `output` variable.

                // We convert the string slice to an owned [String] to prevent
                // lifetime issues, since the slice borrows the input, but we still
                // want to be able to borrow it mutable after this!
                output = Some(path.eat().to_string());
                continue;
            } else {
                return Err("`--out` expects 1 argument, none found".into());
            }
        }

        // A subcommand.
        if input.eat_no_dash("subcommand").is_some() {
            subcommand = parse_subcommand(&mut input)?;
            if subcommand.is_none() {
                return Ok(());
            }
            continue;
        }

        // Every branch above ends with a `return` or `continue` statement.
        // Therefore, if we reach this point, none of the above arguments
        // could be parsed.
        if let Some(arg) = input.value_allows_leading_dashes() {
            return Err(format!("Unexpected {:?} argument", arg.eat()));
        }
    }

    if output.is_none() {
        return Err("missing `--out` argument".into());
    }

    dbg!(output);
    dbg!(subcommand);

    Ok(())
}

fn parse_subcommand(input: &mut impl palex::Input) -> Result<Option<Subcommand>, String> {
    let mut subcommand: Option<Subcommand> = None;

    while !input.is_empty() {
        // A help flag that only applies to the subcommand
        if input.eat_one_dash("h").is_some() || input.eat_two_dashes("help").is_some() {
            print_help_for_subcommand();
            return Ok(None);
        }

        if let Some(sub) = parse_rgb(input)? {
            subcommand = Some(sub);
            continue;
        }
    }
    // The `subcommand` variable is only set if the required `rgb`
    // option was provided
    if subcommand.is_none() {
        return Err("subcommand is missing the `--rgb` argument".into());
    }
    Ok(subcommand)
}

fn parse_rgb(input: &mut impl palex::Input) -> Result<Option<Subcommand>, String> {
    let mut subcommand: Option<Subcommand> = None;

    // Required argument with 3 values. They can be comma-separated, e.g.
    // `--rgb=0,70,255`, or appear in the following arguments, e.g.
    // `--rgb 0 70 255`
    if input.eat_two_dashes("rgb").is_some() {
        if input.can_parse_value_no_whitespace() {
            let mut rgb = input.value().ok_or("No RGB value")?.eat().split(',');

            let r = rgb.next().ok_or("No red part")?;
            let r: u8 = r.parse().map_err(|_| "Invalid red number")?;

            let g = rgb.next().ok_or("No green part")?;
            let g: u8 = g.parse().map_err(|_| "Invalid green number")?;

            let b = rgb.next().ok_or("No blue part")?;
            let b: u8 = b.parse().map_err(|_| "Invalid blue number")?;

            subcommand = Some(Subcommand { r, g, b });
        } else {
            let r = input.value().ok_or("No red part")?.eat();
            let r: u8 = r.parse().map_err(|_| "Invalid red number")?;

            let g = input.value().ok_or("No green part")?.eat();
            let g: u8 = g.parse().map_err(|_| "Invalid green number")?;

            let b = input.value().ok_or("No blue part")?.eat();
            let b: u8 = b.parse().map_err(|_| "Invalid blue number")?;

            subcommand = Some(Subcommand { r, g, b });
        }
    }

    Ok(subcommand)
}

fn print_help() {
    println!("Help for the palex example");
}

fn print_help_for_subcommand() {
    println!("Help for the subcommand");
}
