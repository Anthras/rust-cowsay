# rust-cowsay
👋 This is planned to be a simplified implementation of cowsay 🐮 to help me (and anyone else) learn rust and github.
<pre>
 _________________________
< hello cowsay from Rust! >
 -------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
</pre>


Planned usage for this command-line program is ` ./rust-cowsay "hello world!" --lines --character `

* `/src/` is the folder which houses the source code
* `Cargo.lock` is a saved state of the last working build
* `Cargo.toml` is the file which details the dependencies
* `LICENSE` is the MIT License
* `README.md` is this file, written in markdown 

Implemented features:
* ascii of the original cowsay cow
* speech bubble width based on input length

Todo:
* add ascii rust mascot
* add ascii paperclip
* consider `enum` for various ascii characters
* accept argument for setting multiple lines manually
* parse input for words if length is greater than maximum length and break input into multiple lines
