#[no_mangle]
pub extern "C" fn tokenize_document(document: String) -> Vec<String> {
    let mut res = Vec::new();
    let chars = document.chars();
    for c in chars {
        // Ignored
        if c == '\n' || c =='\t' || c ==' ' || c ==',' {
            continue;
        }
        // Control
        if c == '{' || c =='}' || c =='(' || c ==')'||c ==':' || c =='[' || c == ']' {
            res.push(c.to_string());
            continue;
        }
    }
    return res;
}