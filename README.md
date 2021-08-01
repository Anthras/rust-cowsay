# rust-cowsay
This is planned to be a simplified implementation of cowsay to help me (and anyone else) learn rust and github.

`__________________________`\
`< hello cowsay using Rust! >`\
` --------------------------`\
`        \   ^__^`\
`         \  (oo)\_______`\
`            (__)\       )\/\`\
`                ||----w |`\
`                ||     ||`\


Planned usage for this command-line program is ` ./rust-cowsay "hello world!" `

* `/src/` is the folder which houses the source code
* `Cargo.lock` is a saved state of the last working build
* `Cargo.toml` is the file which details the dependencies
* `LICENSE` is the MIT License
* `README.md` is this file, written in markdown 

Todo includes:
* add ascii of original cow
* add rust mascot
* speech bubble width based on input length
* accept argument for setting multiple lines manually
* parse input for words if length is greater than maximum length and break input into multiple lines
