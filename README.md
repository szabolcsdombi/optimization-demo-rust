# optimization-demo-rust

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

```
----------------------------------------------------------------------------------------------------------
Name (time in ns)            Mean            StdDev                Median           OPS (Kops/s)
----------------------------------------------------------------------------------------------------------
test_optimized_code      321.4845 (1.0)      0.0799 (1.0)        321.4586 (1.0)       3,110.5702 (1.0)
test_rust_code           510.7381 (1.59)     0.1272 (1.59)       510.7077 (1.59)      1,957.9508 (0.63)
test_python_code       1,148.4865 (3.57)     3.4606 (43.32)    1,148.1396 (3.57)        870.7112 (0.28)
----------------------------------------------------------------------------------------------------------
```
