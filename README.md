# optimization-demo-rust

This is an extension to my original [optimization-demo](https://github.com/szabolcsdombi/optimization-demo) article.

The previous article was about optimizing a tiny bit of Python code by replacing it with its C++ counterpart.

In this article we will replace the C++ code with a call into a library built with Rust.

Let's get started by implementing the opening handshake in Rust.

```rust
use sha1::Sha1;
use sha1::Digest;

pub fn sec_websocket_accept(key: &str) -> String {
    let mut concat_key = String::with_capacity(key.len() + 36);
    concat_key.push_str(&key[..]);
    concat_key.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let hash = Sha1::digest(concat_key.as_bytes());
    base64::encode(hash.as_slice())
}
```

The Python we are using is [cpython](https://github.com/python/cpython), hence the name suggests the interpreter is implemented in C and it can call into C/C++ code very easily.
Luckily Rust can expose a C api and build a static library accessible from C/C++.
To do so we need to add an exported method.

```rust
#[no_mangle]
pub extern fn rust_accept(key: *const u8, result: *mut u8) {
    unsafe {
        let source = std::slice::from_raw_parts(key, 24);
        let source_str = std::str::from_utf8_unchecked(source);
        let modified_str = sec_websocket_accept(source_str);
        let dest = std::slice::from_raw_parts_mut(result, 28);
        dest[..28].copy_from_slice(modified_str.as_bytes());
    }
}
```

And on the C++ side we can add an extern function.

```c++
extern "C" void rust_accept(const char * key, char * result);
```

Calling this function from C++ will call into our Rust code.

```c++
PyObject * meth_rust_accept(PyObject * self, PyObject * arg) {
    char result[28];
    Py_ssize_t len = 0;
    const char * key = PyUnicode_AsUTF8AndSize(arg, &len);
    if (!key || len != 24) {
        PyErr_SetString(PyExc_ValueError, "invalid key");
        return NULL;
    }
    rust_accept(key, result);
    return PyUnicode_FromStringAndSize(result, 28);
}
```

Now we have two methods available:

- the first one calls `sec_websocket_accept()` implemented in C++
- the second calls `sec_websocket_accept()` implemented in Rust

All the other code, including validation, is identical.

We can now extend the [test.py](test.py) and see the results.

```py
from mymodule import c_accept, rust_accept


def test_optimized_code(benchmark):
    assert benchmark(c_accept, 'dGhlIHNhbXBsZSBub25jZQ==') == 's3pPLMBiTxaQ9kYGzzhZRbK+xOo='


def test_rust_code(benchmark):
    assert benchmark(rust_accept, 'dGhlIHNhbXBsZSBub25jZQ==') == 's3pPLMBiTxaQ9kYGzzhZRbK+xOo='
```

## Results

```
--------------------------------------------------------------------------------------------------------
Name (time in ns)           Mean            StdDev              Median            OPS (Mops/s)
--------------------------------------------------------------------------------------------------------
test_optimized_code     207.8409 (1.0)      0.2554 (1.83)     207.8076 (1.0)            4.8114 (1.0)
test_rust_code          332.9054 (1.60)     0.1394 (1.0)      332.8895 (1.60)           3.0039 (0.62)
test_python_code        913.2048 (4.39)     3.5578 (25.52)    912.7434 (4.39)           1.0950 (0.23)
--------------------------------------------------------------------------------------------------------
```

## Conclusion

According to the results the C++ implementation is around 1.6x times faster than the Rust implementation.
The two should be on par, except that that Rust needs additional work to expose a C api.

## Notes

I want to highlight that we are not comparing these languages directly.

The goal is to have a Pythonic method with the following signature:

```py
def websocket_accept(key: str) -> str: ...
```

The method itself will not be implemented in Python, but it will be **available** in Python.

I could not build a Python extension with only using Rust.
I have implemented the simplest way to get the Rust implementation working from Python.
If there is a better way please feel free to make a pull request.
