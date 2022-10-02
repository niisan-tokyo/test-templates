FROM php:8

RUN apt-get update && apt-get install -y git unzip libonig-dev libzip-dev && \
docker-php-ext-install mbstring zip

COPY --from=composer /usr/bin/composer /usr/bin/composer