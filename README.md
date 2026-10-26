# hokun

```
$ docker build -t hokun .
$ docker run --rm -p 8080:3000 hokun
$ docker exec -it hokun /app/hokun migration apply
$ docker exec -it hokun /app/hokun create-user --name NAME
```
