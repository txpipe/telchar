# Zot Registry

A simple registry for storing and retrieving dapps data.


## Usage

**For ARM-based processor**
```bash
  docker compose -f docker-compose.arm.yml up -d
```

**For Intel-based processor**
```bash
  docker compose -f docker-compose.amd.yml up -d
```

**After having the registry running, you can run seed to fill with some data.**

```bash
  cd seed
  cargon run
```
