use cpython::{PyResult, Python, py_module_initializer, py_fn};

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(miscLib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "str_replacer", py_fn!(py, str_replacer(initial_string: String, ch: String, replacing_character: String, occurence: i64)))?;
    Ok(())
});

pub fn str_replacer(_py: Python, initial_string: String, ch: String, replacing_character: String, occurence: i64) -> PyResult<String> {
    let mut result: String = "".to_string();
    let mut count = 0;
    for c in initial_string.chars() {
        if c == ch.chars().nth(0).unwrap() && count != occurence {
            result.push(replacing_character.chars().nth(0).unwrap());
            count += 1;
        } else {
            result.push(c);
        }
    }
    
    Ok(result)
}
