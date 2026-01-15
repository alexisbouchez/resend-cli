# Resend CLI

A command-line interface for interacting with the Resend API. This tool allows users to manage emails, domains, API keys, and other resources through the Resend service from the command line.

## Features

- Send and manage emails
- Manage domains and verify ownership
- Create and manage API keys
- Handle contacts and segments
- Manage templates, topics, and webhooks
- Send and receive broadcast messages
- Manage contact properties

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/resend-cli.git
cd resend-cli

# Build the project
cargo build --release

# The executable will be available at target/release/resend
```

## Configuration

Before using the CLI, you need to configure it with your Resend API key:

```bash
# Using the config command
resend config --api-key YOUR_API_KEY

# Or set the environment variable
export RESEND_API_KEY=YOUR_API_KEY
```

The CLI will first check for the `RESEND_API_KEY` environment variable, and if not found, it will look for the configuration file at `~/.resend-cli/config.json`.

## Usage

### Global Options

```bash
resend [COMMAND] [OPTIONS]
```

### Commands

#### Config

Configure the CLI with your API key:

```bash
resend config --api-key YOUR_API_KEY
```

#### Emails

Manage emails through various subcommands:

```bash
# Send an email
resend emails send --from sender@example.com --to recipient@example.com --subject "Hello" --html "<h1>Hello!</h1>"

# Save an email as a draft (saves to a local draft file)
resend emails draft --from sender@example.com --to recipient@example.com --subject "Hello" --html "<h1>Hello!</h1>" --html-file path/to/html_file.html --text-file path/to/text_file.txt

# List emails
resend emails list

# Get a specific email
resend emails get EMAIL_ID

# Cancel a scheduled email
resend emails cancel EMAIL_ID

# Update a scheduled email
resend emails update --scheduled-at "2023-12-31T10:00:00Z" EMAIL_ID

# List email attachments
resend emails attachments EMAIL_ID

# Send a batch of emails from a JSON file
resend emails send-batch path/to/emails.json
```

#### Domains

Manage domains:

```bash
# Create a domain
resend domains create --name example.com

# List domains
resend domains list

# Get a specific domain
resend domains get DOMAIN_ID

# Delete a domain
resend domains delete DOMAIN_ID

# Verify a domain
resend domains verify DOMAIN_ID
```

#### API Keys

Manage API keys:

```bash
# Create an API key
resend api-keys create --name "My API Key"

# List API keys
resend api-keys list

# Delete an API key
resend api-keys delete KEY_ID
```

#### Contacts

Manage contacts:

```bash
# Create a contact
resend contacts create --email contact@example.com --first-name John --last-name Doe

# List contacts
resend contacts list

# Get a specific contact
resend contacts get CONTACT_ID

# Update a contact
resend contacts update CONTACT_ID --first-name Jane

# Delete a contact
resend contacts delete CONTACT_ID
```

#### Templates

Manage email templates:

```bash
# Create a template
resend templates create --name "Welcome Email" --subject "Welcome!" --html "<h1>Welcome!</h1>"

# List templates
resend templates list

# Get a specific template
resend templates get TEMPLATE_ID

# Update a template
resend templates update TEMPLATE_ID --subject "Updated Subject"

# Delete a template
resend templates delete TEMPLATE_ID
```

#### Segments

Manage segments:

```bash
# Create a segment
resend segments create --name "VIP Customers"

# List segments
resend segments list

# Get a specific segment
resend segments get SEGMENT_ID

# Delete a segment
resend segments delete SEGMENT_ID
```

#### Topics

Manage topics:

```bash
# Create a topic
resend topics create --name "Newsletter" --description "Monthly newsletter"

# List topics
resend topics list

# Get a specific topic
resend topics get TOPIC_ID

# Update a topic
resend topics update TOPIC_ID --description "Updated description"

# Delete a topic
resend topics delete TOPIC_ID
```

#### Webhooks

Manage webhooks:

```bash
# Create a webhook
resend webhooks create --url https://example.com/webhook --events email.opened,email.delivered

# List webhooks
resend webhooks list

# Get a specific webhook
resend webhooks get WEBHOOK_ID

# Delete a webhook
resend webhooks delete WEBHOOK_ID
```

#### Broadcasts

Manage broadcasts:

```bash
# Create a broadcast
resend broadcasts create --name "Summer Sale" --template-id TEMPLATE_ID

# List broadcasts
resend broadcasts list

# Get a specific broadcast
resend broadcasts get BROADCAST_ID

# Update a broadcast
resend broadcasts update BROADCAST_ID --name "Winter Sale"

# Delete a broadcast
resend broadcasts delete BROADCAST_ID

# Send a broadcast
resend broadcasts send BROADCAST_ID
```

#### Contact Properties

Manage contact properties:

```bash
# Create a contact property
resend contact-properties create --name "Subscription Date" --type date

# List contact properties
resend contact-properties list

# Get a specific contact property
resend contact-properties get PROPERTY_ID

# Update a contact property
resend contact-properties update PROPERTY_ID --name "Updated Property"

# Delete a contact property
resend contact-properties delete PROPERTY_ID
```

#### Receiving

Manage received emails:

```bash
# List received emails
resend receiving list

# Get a specific received email
resend receiving get EMAIL_ID
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Building Documentation

```bash
# Generate documentation
cargo doc --open
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a new Pull Request

## License

This project is licensed under the [MIT License](./LICENSE).