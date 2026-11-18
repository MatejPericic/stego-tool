# stego-tool
A command line LSB steganography tool written in Rust.

Example: 

Encoding

```
stego-tool.exe encode --depth 1 --image cover.jpg --message message.txt --output out.png
```

Decoding

```
stego-tool.exe decode --depth 1 --image out.png --output message.txt
```
