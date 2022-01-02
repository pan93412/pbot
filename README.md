# pbot

pan93412's extensible user bot, which is full-documented, engineered and based on Actor model.

## Usage

### Run

```sh
cargo run --release [--features <modules id>]
```

### Configure

1. Copy `.env.example` to `.env`
2. Configure it according to the instruction.

## Hacking

### Build

```sh
cargo build [--features <modules id>]
```

### Docs

```sh
cargo doc [--features <modules id>]
```

### Run

```sh
cargo run [--features <modules id>]
```

## Modules

| Modules ID   | Modules Name    | Description                                                                               | Enable by Default |
| ------------ | --------------- | ----------------------------------------------------------------------------------------- | ----------------- |
| `fwdmod`     | `FwdModule`     | Simply forward the message to your specified chat with `!cufwd`.                          | ✅                |
| `getinfomod` | `GetInfoModule` | Get the information of the message. For debugging purpose.                                | ❌                |
| `addrankmod` | `AddRankModule` | You can add rank for every member you administrated without giving the actual permission. | ❌                |

## Authors

- pan93412, 2021
