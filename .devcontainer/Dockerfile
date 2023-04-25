FROM python:3-slim

# Prepare tools.
RUN apt-get update
RUN apt-get install -y --no-install-recommends git curl nano vim

# Prepare dependencies.
RUN pip3 install poetry; \
    pip3 install git+https://github.com/extism/cli

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Color the shell.
RUN { \
    echo -ne "\nexport PS1=\"${debian_chroot:+($debian_chroot)}"; \
    echo -n "\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]"; \
    echo -n "\$ \""; \
} >> ~/.bashrc
