# Png Msg
Encode message into PNG image
## Why?
Chill project I did during the weekend

## Usage
```bash
png-msg 0.1.0

USAGE:
    png-msg.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decodes message from specified chunk of PNG file
    encode    Encodes message into PNG file
    help      Print this message or the help of the given subcommand(s)
    print     Prints all chunks of PNG file
    remove    Removes specified chunk from PNG file
```
### What is chunk?
Chunk is just a bunch of bytes [organazied in some special way](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html). What `png-me` wants you to specify is `chunk_type`, according to official docs this is
> A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal). However, encoders and decoders must treat the codes as fixed binary values, not character strings. For example, it would not be correct to represent the type code IDAT by the EBCDIC equivalents of those letters. Additional naming conventions for chunk types are discussed in the next section.

Simply put this is just four-letter word where the fourth letter is capitalized ¯\\_(°ペ)\_/¯
