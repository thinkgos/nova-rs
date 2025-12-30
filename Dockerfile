FROM rust:1.92.0 as builder 
WORKDIR /app

# 更新阿里云的bullseye版本包源
RUN update-ca-certificates && \
    echo "deb https://mirrors.aliyun.com/debian/ bullseye main non-free contrib" > /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian/ bullseye main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb https://mirrors.aliyun.com/debian-security/ bullseye-security main" >> /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian-security/ bullseye-security main" >> /etc/apt/sources.list && \
    echo "deb https://mirrors.aliyun.com/debian/ bullseye-updates main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian/ bullseye-updates main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb https://mirrors.aliyun.com/debian/ bullseye-backports main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian/ bullseye-backports main non-free contrib" >> /etc/apt/sources.list && \
    apt update && \
    apt install lld clang -y

COPY . .

RUN cargo build --release -p app

FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt update -y && \
    apt install ca-certificates -y && \ 
    update-ca-certificates && \
    echo "deb https://mirrors.aliyun.com/debian/ bullseye main non-free contrib" > /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian/ bullseye main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb https://mirrors.aliyun.com/debian-security/ bullseye-security main" >> /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian-security/ bullseye-security main" >> /etc/apt/sources.list && \
    echo "deb https://mirrors.aliyun.com/debian/ bullseye-updates main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian/ bullseye-updates main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb https://mirrors.aliyun.com/debian/ bullseye-backports main non-free contrib" >> /etc/apt/sources.list && \
    echo "deb-src https://mirrors.aliyun.com/debian/ bullseye-backports main non-free contrib" >> /etc/apt/sources.list && \
    apt-get update -y && \
    apt-get install -y --no-install-recommends openssl && \
    # Clean up
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app app
COPY conf conf

ENV NOVA_DEPLOY_MODE prod

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./app"]