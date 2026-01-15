# Resend CLI

CLI for the Resend API. Requires `RESEND_API_KEY` env var or run `resend config --api-key <key>`.

## Commands

```
resend emails send --from <email> --to <email> --subject <text> [--html <html>] [--text <text>]
resend emails list
resend emails get <id>
resend emails cancel <id>
resend emails draft --from <email> --to <email> --subject <text> [--html <html>]

resend domains list
resend domains create --name <domain>
resend domains get <id>
resend domains verify <id>
resend domains delete <id>

resend api-keys list
resend api-keys create --name <name>
resend api-keys delete <id>

resend contacts list --audience-id <id>
resend contacts create --audience-id <id> --email <email>
resend contacts get --audience-id <id> --id <id>
resend contacts update --audience-id <id> --id <id>
resend contacts delete --audience-id <id> --id <id>

resend templates list
resend templates create --name <name>
resend templates get <id>
resend templates update <id>
resend templates delete <id>

resend broadcasts list
resend broadcasts create --audience-id <id> --from <email> --subject <text>
resend broadcasts send <id>

resend webhooks list
resend webhooks create --endpoint <url> --events <events>

resend segments list --audience-id <id>
resend topics list --audience-id <id>
resend contact-properties list --audience-id <id>
resend receiving list
```

## Install

```
cargo install resend-cli
```
