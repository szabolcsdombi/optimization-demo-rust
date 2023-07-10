# c-rust-python

This is an extension to my original [optimization-demo](https://github.com/szabolcsdombi/optimization-demo) article.

The previous article was about optimizing a tiny bit of Python code by replacing it with its C++ counterpart.

In this article we will replace the C++ code with a call into a library built with Rust.

## Notes

I want to highlight that we are not comparing these languages directly.

The goal is to have a Pythonic method with the following signature:

```py
def websocket_accept(key: str) -> str: ...
```

The method itself will not be implemented in Python, but it will be **available** in Python.

## Results

```py
----------------------------------------------------------------------------------------------------------
Name (time in ns)            Mean            StdDev                Median           OPS (Kops/s)          
----------------------------------------------------------------------------------------------------------
test_optimized_code      336.9099 (1.0)      0.1530 (1.67)       336.8627 (1.0)       2,968.1530 (1.0)    
test_rust_code           539.2216 (1.60)     0.0917 (1.0)        539.2166 (1.60)      1,854.5250 (0.62)   
test_python_code       1,288.7680 (3.83)     1.3803 (15.05)    1,288.8969 (3.83)        775.9349 (0.26)   
----------------------------------------------------------------------------------------------------------
```
