# Command line to print binary content in desired radix

```
Usage: binary [OPTIONS] [INPUT] [OUTPUT]

Arguments:
  [INPUT]   Path to a binary file [default: stdin]
  [OUTPUT]  Path to write the text output [default: stdout]

Options:
  -a, --address                  Show the address of the first byte of each line
  -t, --text                     Show the corresponding ascii character
  -r, --radix <RADIX>            Numerical base for the bytes value [default: bin] [possible values: bin, oct, dec, hex]
  -s, --select <SELECT>          What range of the input to show, as N..N where N is an optional integer [default: ..]
  -l, --line-width <LINE_WIDTH>  How many bytes per line [default: 8]
  -b, --break-on <BREAK_ON>      Start a new line when a given byte value is found
  -c, --colored                  If the output should you ansi escapes sequences
  -h, --help                     Print help
```

```
$ ./binary Cargo.toml -at -r hex -l 20 -b 10

000000  5b 70 61 63 6b 61 67 65 5d 0a                                [package]           
00000a  6e 61 6d 65 20 3d 20 22 62 69 6e 61 72 79 22 0a              name = "binary"     
00001a  76 65 72 73 69 6f 6e 20 3d 20 22 30 2e 31 2e 30 22 0a        version = "0.1.0"   
00002c  65 64 69 74 69 6f 6e 20 3d 20 22 32 30 32 31 22 0a           edition = "2021"    
00003d  0a                                                                               
00003e  5b 64 65 70 65 6e 64 65 6e 63 69 65 73 5d 0a                 [dependencies]      
00004d  63 6c 61 70 20 3d 20 7b 20 76 65 72 73 69 6f 6e 20 3d 20 22  clap = { version = "
000061  34 2e 35 2e 32 39 22 2c 20 66 65 61 74 75 72 65 73 20 3d 20  4.5.29", features = 
000075  5b 22 64 65 72 69 76 65 22 5d 20 7d 0a                       ["derive"] }        
```
