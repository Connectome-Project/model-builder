# Connectome-Project

!!!!Under construction.

This projects aims to realize a graph-based connectome with planned simplification steps.
So far the builder is created with an in-memory storage. A re-implementation will be on way with a graph database to give it a simplified logic and also to store the data in a more robust way.

## Crates:

### Connectome common:

This is the initial implementation of an in-memory implementation of the graph builder step.

### Connectome graph:

Start of the reimplementation of the builder step with an actual database implemenation.

## Python interoperability:

Pulling
For python interoperability check out the user guide for [pyo3](https://pyo3.rs/v0.19.0/) which is using [maturin](https://www.maturin.rs/tutorial.html) in the background.

Steps to start rust code from python

1. cd connectome_python_binding
2. Create venv if doesn't exist: pyhton -m venv .venv
3. source .env/bin/activate
4. pip install -r requirements.txt
5. maturin develop
6. launch using debug of vs code (f5) or by python -m python/main.py

## Starting the graph application and database:

1. docker compose up
