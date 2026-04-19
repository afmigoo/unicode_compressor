FROM nginx:latest
RUN rm /usr/share/nginx/html/*
COPY ./web /usr/share/nginx/html
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
CMD ["nginx", "-g", "daemon off;"]
