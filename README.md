# jingjiang

## Setup

### generate prisma client

```console
cargo prisma generate
```

### setup env file

```console
cp .env.exampple .env
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

