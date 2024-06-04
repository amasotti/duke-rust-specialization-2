# GPU Stress test (PyTorch bindings for Rust)

__Prerequisites__: Read carefully [here](https://github.com/LaurentMazare/tch-rs/blob/main/README.md)

You need to have a CUDA enabled GPU to run this code and install `libtorch` or `pytorch` via anaconda on your system.

If we use Pytorch installed via conda, we need to set these env vars:

~~~bash
LIBTORCH_USE_PYTORCH=1
LIBTORCH_BYPASS_VERSION_CHECK=1 # In case the version is not the expected one (e.g. I have 2.3.1, the script requires 2.3.0)
~~~

## See also

- [PyTorch Docs](https://pytorch.org/get-started/locally/)