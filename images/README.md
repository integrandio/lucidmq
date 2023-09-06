# Base Images

In order to save on build time, we build base images that have all of our dependencies built in in a multistep build process. 

## How to build base files locally

These commands are run from the LucidMQ root directory.

Rust Base Image
```
docker build -f images/RustBase.Dockerfile -t rust-base .
```

Python Base Image
```
docker build -f images/PythonBase.Dockerfile -t python-base .
```

Go Base Image
```
docker build -f images/GoBase.Dockerfile -t go-base .

## To write it to the registry
```
docker build -f images/RustBase.Dockerfile -t registry.nocaply.com/go-base:latest .

docker build -f images/PythonBase.Dockerfile -t registry.nocaply.com/python-base:latest .

docker build -f images/GoBase.Dockerfile -t registry.nocaply.com/go-base:latest . 
```