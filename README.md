# Docker with Rust #

A containerized development environment for building [Rust](https://www.rust-lang.org/ "Rust Programming Language") programs.

## Getting Started ##

Start by downloading the [Latest Release](https://github.com/BeautifulTovarisch/rust-template/releases/latest "Rust Development Package")

### Environment Variables ###

You'll need to make a file called `.env` in the **same directory** as your docker-compose.yml. Your file should contain values for the following variables:

```
# Place the value right next to the "=" sign, no spaces!
# e.g. GID=1000

GID=
UID=
```

It's important that these are the values of your host group and user id, respectively.

**Tip** You can determine your uid and gid by running `id` in your terminal.

### Rust Image ###

We'll be building and running the Rust container in this step.

Run the following commands. You should be in the directory with `docker-compose.yml` inside.

1. `docker-compose build rust`
    - This builds the container image

2. `docker-compose up rust`
    - Brings up your development container

### Development ###

You should be able to see the output of your tests and any compiler errors or warnings in your terminal. A utility will rerun your tests and check your code when you update files.
