static TRANSLATES: &[(&str, &str)] = &[
    ("invalid-amount", "invalid amount"),
    ("too-poor", "you dont have enough")
];

pub(crate) fn translate(str: &str) -> &str {
    for (k, v) in TRANSLATES {
        if k == &str {
            return v;
        }
    }

    return "key not found";
}