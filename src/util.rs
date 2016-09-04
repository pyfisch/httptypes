fn is_tchar(c: char) -> bool {
    matches!(c, '!' | '#' | '$' | '%' | '&' | '\'' | '*'
    | '+' | '-' | '.' | '^' | '_' | '`' | '|' | '~'
    | '0'...'9' | 'A'...'Z' | 'a'...'z')
}

pub fn is_token(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().all(is_tchar)
}
