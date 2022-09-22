
use pyo3::prelude::*;

/// Check the phone number for length/prefix and return without country code.
#[pyfunction]
fn check_number(s: String, country_code: String, length: usize) -> PyResult<String> {
    let mut long = s.len();
    let mut phone = String::from("");
    if long >= length {
        let code_length = country_code.len();
        let max_length = code_length + length;
        let mut is_head = true;
        let mut count = 0;
        for c in s.chars() {
            if is_head {
                if (c == '0') || (c == '+') {
                    long -= 1;
                } else if (count > max_length) || (!c.is_ascii_digit()) {
                    phone.clear();
                    break;
                } else {
                    phone.push(c);
                    count += 1;
                    is_head = false
                }
            } else {
                if (count > max_length) || (!c.is_ascii_digit()) {
                    phone.clear();
                    break;
                } else {
                    phone.push(c);
                    count += 1;
                }
            }
        }
        if count == max_length {
            for _ in 0..code_length {
                phone.remove(0);
            }
        }
    }
    Ok(phone)
}

/// A Python module implemented in Rust.
#[pymodule]
fn phone_validator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_number, m)?)?;
    Ok(())
}
