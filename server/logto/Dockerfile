FROM alpine/git as git

WORKDIR /etc

RUN git clone https://github.com/logto-io/logto.git

###### [STAGE] Build ######
FROM svhd/logto as builder

###### [STAGE] Seal ######
FROM nginx as app

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install nodejs npm -y

WORKDIR /etc/logto
COPY --from=builder /etc/logto .
RUN mkdir -p /etc/logto/packages/cli/alteration-scripts && chmod g+w /etc/logto/packages/cli/alteration-scripts
EXPOSE 8080
COPY nginx.conf /etc/nginx/nginx.conf

COPY run.sh .
RUN chmod +x run.sh

CMD ["sh", "-c", "./run.sh"]
