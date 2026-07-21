# modfolder-toggler

CLI for the [Prevent Transparency mod](https://gamebanana.com/mods/611267) (Wuthering Waves). Enables a chosen set of character folders under `Shaders` and disables the rest by renaming them with a `DISABLED ` prefix.

Either download a binary or build it from source yourself with
```
cargo build --release
```
and move the binary to the root folder which contains the Shaders folder.

The user is prompted with 4 options which they may select by typing the corresponding number(1-4).
Enable all
Enable selected
Disable selected
Disable all

Matching is case-insensitive substring.

MIT
