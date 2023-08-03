static TRANSLATES: &[(&str, &str)] = &[
    ("invalid-amount", "invalid ris amount"),
    ("too-poor", "you dont have enough ris"),
    ("user-not-found", "user not found, the user must have used <@568163802907148307> atleast once")
];

pub(crate) fn translate(str: &str) -> &str {
    for (k, v) in TRANSLATES {
        if k == &str {
            return v;
        }
    }

    return "key not found";
}