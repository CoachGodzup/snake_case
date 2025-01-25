#https://www.docker.com/blog/cross-compiling-rust-code-for-multiple-architectures/

FROM rust:latest 

#TODO build for windows
#RUN apt update &amp;&amp; apt upgrade -y 
#RUN apt install -y g++-mingw-w64-x86-64 
 
#RUN rustup target add x86_64-pc-windows-gnu 
#RUN rustup toolchain install stable-x86_64-pc-windows-gnu 
 
#ENV PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-w64-mingw32
#ENV PKG_CONFIG_PATH=x86_64-unknown-linux-gnu

#CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"]

WORKDIR /app 
CMD ["cargo", "build", "-r"],

