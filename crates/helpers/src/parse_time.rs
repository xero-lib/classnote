pub fn parse_time(input: &str) -> Result<(u8, u8, u8), &str>{
    let split = input.split(" ").collect::<Vec<&str>>();
    
    if split.len() > 2 || (split.len() == 2 && !matches!(split.last().unwrap().to_ascii_lowercase().as_str(), "am" | "pm")) {
        return Err("Invalid input: \"{input}\". Please use one of the specified formats.")
    }

    let mut hms = split.first().unwrap().split(":");
    let Ok(hours) = u8::from_str_radix(hms.next().unwrap(), 10) else {
        return Err("Invalid hours entry. Please try again.");
    };

    let Ok(mins)  = u8::from_str_radix(hms.next().unwrap_or("0"), 10) else {
        return Err("Invalid minutes entry. Please try again.");
    };

    let Ok(secs)  = u8::from_str_radix(hms.next().unwrap_or("0"), 10) else {
        return Err("Invalid seconds entry. Please try again.");
    };

    if ((split.len() == 1 && hours > 24) || (split.len() == 2 && hours > 12)) || mins > 60 || secs > 60 {
        return Err("Please enter a valid time.");
    }

    return Ok((
        match (hours, split.last().map(|s| s.to_ascii_lowercase()).as_deref()) {
            (12, Some("am")) => 0,
            (12, Some("pm")) => 12,
            (h, Some("pm")) => h + 12,
            (h, _)          => h, // 12 hour or any other am
        }, mins, secs));
}