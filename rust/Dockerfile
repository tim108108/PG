FROM ubuntu:latest
WORKDIR /app
COPY . .

RUN apt update \
    && apt install -y python3 pip \
    && pip install -r requirements.txt