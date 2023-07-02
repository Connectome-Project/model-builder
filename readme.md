# Connectome-Project

## Python interoperability:

Pulling
For python interoperability check out the user guide for [pyo3](https://pyo3.rs/v0.19.0/) which is using [maturin](https://www.maturin.rs/tutorial.html) in the background.

Steps to start rust code from python

2. cd connectome_common
3. Create venv if doesn't exist: pyhton -m venv .venv
4. source .env/bin/activate
5. pip install -r requirements.txt
6. maturin develop
7. launch using debug of vs code (f5) or by python -m python/main.py
