# jingjiang

## Setup

### generate prisma client

```console
cargo prisma generate
```

### create database

```console
sqlite3 sqlite.db

# with sqlite3 
> .databases
> .quit
```

### migrate

```
cargo prisma migrate deploy
```

