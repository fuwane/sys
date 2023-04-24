FROM python:3-slim

RUN apt-get update
RUN apt-get install -y --no-install-recommends git curl

RUN pip3 install poetry; \
    pip3 install git+https://github.com/extism/cli

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN apt-get remove -y git curl; apt-get clean
RUN rm -rf /var/lib/apt/lists/*