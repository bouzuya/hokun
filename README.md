# hokun

```
$ # show migration help
$ cargo run --bin db migration

$ docker build -t hokun .
$ docker run --rm -p 8080:3000 hokun
$ docker exec -it hokun /app/db migration apply
$ docker exec -it hokun /app/hokun create-user --name NAME
```
