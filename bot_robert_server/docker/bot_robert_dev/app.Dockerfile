FROM rust:1.53

# Rebuild code on each start to apply code changes.
CMD cargo install --path /var/app/bot_robert_server/; bot_robert_server
