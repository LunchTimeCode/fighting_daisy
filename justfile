import 'justfiles/linting.just'

image_name := "ghcr.io/lunchtimecode/handball"
SERVER_PORT := "9999"


run *args:
    cargo run -- {{args}}

s:

w:
    cargo watch --ignore 'assets/css' -s 'just run'