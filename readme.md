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

## Starting the graph application and database:

1. docker compose up
2. In the browser open: [http://localhost:8300/explorer](http://localhost:8300/explorer)
3. Connect to the default: http://localhost:8200/graphql
4. Run queries against the db or observe its state.
5. Start the connectome_graph application
