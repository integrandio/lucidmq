# Base Images

In order to save on build time, we build base images that have all of our dependencies built in. 

## How to build locally

Rust Base Image
```
docker build -f base_images/RustBase.Dockerfile -t rust-base .
```

Python Base Image
```
docker build -f base_images/PythonBase.Dockerfile -t python-base .
```