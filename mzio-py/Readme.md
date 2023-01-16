# Python bindings of rusteomics-proteomis-io

## Install
Needs to be published   
`pip install rusteomics_proteomics_io_py`

## Install dependencies for development
1. Install (Ana|mini){0,1}conda
2. `conda env create -f environment.yaml`
3. `conda activate rusteomics_proteomics_io`

## Build
`maturin develop` (or `build`)

## Testing
`python -m unittest`