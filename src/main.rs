fn likes(names: &[&str]) -> String {
    if names.is_empty() {
        format!("no one likes this")
    } else if names.len() == 1 {
        format!("{}, likes this", names[0])
    } else if names.len() == 2 {
        format!("{} and {} like this", names[0], names[1])
    } else if names.len() == 3 {
        format!("{}, {} and {} like this", names[0], names[1], names[2])
    } else {
        format!(
            "{}, {}, and {} others like this",
            names[0],
            names[1],
            names.len() - 2
        )
    }
}
