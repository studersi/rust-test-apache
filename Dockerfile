FROM rust

ARG USERNAME="rustacian"

RUN apt-get update
RUN apt-get -y install --no-install-recommends apache2-dev llvm librust-bindgen-dev

RUN groupadd -r ${USERNAME} && useradd -s /bin/bash -g ${USERNAME} ${USERNAME}
USER ${USERNAME}

WORKDIR /home/${USERNAME}
