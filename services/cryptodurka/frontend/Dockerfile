FROM nginx:1.25-alpine

RUN mkdir -p /var/www
COPY . /var/www

RUN rm /etc/nginx/conf.d/*
ADD ngix/ngix.conf /etc/nginx/conf.d

CMD ["nginx", "-g", "daemon off;"]