# pyo3 & maturin

## env in conda

```shell
# https://pyo3.rs/v0.17.3/ this is conda
conda env create
# done
conda activate test_pyo3
pip install maturin

```

## develop

```shell
maturin develop
```

test:

```shell
python

>>> import test_pyo3
>>> test_pyo3.sum_as_string(5, 20)
'25'
>>> exit()

```

## build and publish

```shell
#maturin build
maturin build -f -r

```

```shell
maturin publish
```

- https://stackoverflow.com/questions/69585768/pyo3-maturin-publish-command-doesnt-upload-readme-and-license-to-pypi

- https://blog.csdn.net/KINGEH/article/details/127164521
- [Attempting to include the sdist output tarball [...] into itself](https://github.com/PyO3/maturin-action/issues/99) : [CI.yml is Ture](https://github.com/LiberTEM/LiberTEM-dectris-rs/blob/main/.github/workflows/CI.yml)
    
