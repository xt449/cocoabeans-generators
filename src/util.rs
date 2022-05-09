const NAMESPACE_GIVEN_PREFIX: &str = "minecraft:";

/// A namespace has the form "minecraft:stone.hit_soft"
///
/// A file name has the form "stone.hit_soft"
pub fn namespace_to_file_name(original: &String) -> String {
    return original.trim_start_matches(NAMESPACE_GIVEN_PREFIX)
        // takes care of weird "worldgen/*" case
        .replace('/', "_");
}

/// A Namespace has the form "minecraft:stone.hit_soft"
///
/// A rust identifier has the form "r#stone·hit_soft"
///
/// We prefix with the rust identifier escape sequence ("r#") to avoid having to deal with compilation issues on certain names.
/// (ie. "minecraft:match" in the "minecraft:motive" registry)
pub fn namespace_to_rust_identifier(original: &str) -> String {
    return "r#".to_owned() + original.trim_start_matches(NAMESPACE_GIVEN_PREFIX).replace('.', "·").as_str();
}

/// A namespace has the form "minecraft:stone.hit_soft"
///
/// A pascal case indentifier has the form "Stone·HitSoft"
pub fn namespace_to_pascal_case(original: &String) -> String {
    let original = original.trim_start_matches(NAMESPACE_GIVEN_PREFIX);

    let mut word_start: bool = true;

    return original.chars().map(|c|
        if word_start {
            word_start = false;
            return c.to_ascii_uppercase();
        } else if c == '.' {
            word_start = true;
            return '·';
        } else if c == '/' {
            // takes care of weird "worldgen/*" case
            word_start = true;
            return '_';
        } else {
            if c == '_' {
                word_start = true;
            }
            return c;
        }
    ).filter(|c| *c != '_').collect();
}