FROM gcr.io/distroless/python3-debian11:latest

RUN pip3 install poetry && \
    pip3 install git+https://github.com/extism/cli

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y