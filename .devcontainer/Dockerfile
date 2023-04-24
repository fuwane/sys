FROM python:3-slim

RUN python -m pip install poetry && \
    python -m pip install git+https://github.com/extism/cli

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y