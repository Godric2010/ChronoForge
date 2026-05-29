pub(crate) fn modify_name_with_count_of_equals(
    target_name: &str,
    other_names: &[String],
) -> String {
    let mut next_suffix = 0;

    for other_name in other_names {
        if other_name == target_name {
            next_suffix = next_suffix.max(1);
            continue;
        }

        let Some(suffix_part) = other_name
            .strip_prefix(target_name)
            .and_then(|name| name.strip_prefix('('))
            .and_then(|name| name.strip_suffix(')'))
        else {
            continue;
        };

        if let Ok(suffix) = suffix_part.parse::<u32>() {
            next_suffix = next_suffix.max(suffix + 1);
        }
    }

    if next_suffix == 0 {
        target_name.to_string()
    } else {
        format!("{}({})", target_name, next_suffix)
    }
}
