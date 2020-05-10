FROM node:12.7-alpine AS build

WORKDIR /app

COPY ./client/package.json .

RUN npm install

COPY ./client/ .

RUN npm run build --prod --deploy-url=/

FROM nginx:alpine

COPY --from=build /app/nginx/nginx.conf /etc/nginx/nginx.conf

RUN rm -rf /usr/share/nginx/html/*

COPY --from=build /app/dist /usr/share/nginx/html

ENTRYPOINT ["nginx", "-g", "daemon off;"]