# -----------------------------------------------
# Stage 1: ビルドステージ
# -----------------------------------------------
FROM rust:latest AS builder

# ホストユーザーの UID/GID に合わせてコンテナ内ユーザーを作成する
# デフォルトは 1000:1000（Linux の一般ユーザー）
# 例: docker compose build --build-arg USER_ID=$(id -u) --build-arg GROUP_ID=$(id -g)
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN groupadd -g ${GROUP_ID} appgroup \
    && useradd -u ${USER_ID} -g appgroup -m -s /bin/bash appuser

WORKDIR /app

# appuser が /app 配下を書き込めるように権限を設定
RUN chown -R appuser:appgroup /app

# cargo のキャッシュディレクトリも appuser に帰属させる
RUN mkdir -p /usr/local/cargo/registry \
    && chown -R appuser:appgroup /usr/local/cargo

# 以降の操作は appuser として実行
USER appuser

# 依存関係のキャッシュ最適化のために先に Cargo.toml / Cargo.lock だけコピー
COPY --chown=appuser:appgroup Cargo.toml Cargo.lock* ./

# ダミーの main.rs を作成してビルドキャッシュを温める
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

# 実際のソースをコピーしてビルド
COPY --chown=appuser:appgroup src ./src
# タイムスタンプを更新してキャッシュ対象として認識させる
RUN touch src/main.rs && cargo build --release

# -----------------------------------------------
# Stage 2: 実行ステージ（最小イメージ）
# -----------------------------------------------
FROM debian:bookworm-slim AS runtime

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 実行ステージにも同じユーザーを作成
RUN groupadd -g ${GROUP_ID} appgroup \
    && useradd -u ${USER_ID} -g appgroup -m -s /bin/bash appuser

WORKDIR /app
RUN chown -R appuser:appgroup /app

# ビルド済みバイナリのみコピー
COPY --from=builder --chown=appuser:appgroup /app/target/release/iris .

USER appuser

EXPOSE 3000

CMD ["./iris"]
