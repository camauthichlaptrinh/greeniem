pub fn vnd(amount: i64) -> String {
    let s = amount.abs().to_string();
    let mut grouped = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            grouped.push('.');
        }
        grouped.push(c);
    }
    let grouped: String = grouped.chars().rev().collect();
    if amount < 0 {
        format!("-{grouped}₫")
    } else {
        format!("{grouped}₫")
    }
}
