# Base Images

In order to save on build time, we build base images that have all of our dependencies built in in a multistep build process. 

## How to build base files locally

These commands are run from the LucidMQ root directory.

Rust Base Image
```bash
docker build -f images/RustBase.Dockerfile -t rust-base .

# All the docker images use references from the registry here, build your image like so to be consistent
docker build -f images/RustBase.Dockerfile -t registry.nocaply.com/rust-base:latest .
```

Python Base Image
```bash
docker build -f images/PythonBase.Dockerfile -t python-base .

# All the docker images use references from the registry here, build your image like so to be consistent
docker build -f images/PythonBase.Dockerfile -t registry.nocaply.com/python-base:latest .
```

Go Base Image
```bash
docker build -f images/GoBase.Dockerfile -t go-base .

# All the docker images use references from the registry here, build your image like so to be consistent
docker build -f images/GoBase.Dockerfile -t registry.nocaply.com/go-base:latest . 
```