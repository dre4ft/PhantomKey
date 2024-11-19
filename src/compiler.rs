use crate::token::Token;

pub fn compile(tokens: Vec<Token>) -> Vec<u8> {
    let mut binary_output = Vec::new();

    for token in tokens {
        let report = token.to_hid_report();
        binary_output.extend(report);
    }

    binary_output
}
