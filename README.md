# bin2src

> **bin2src** is a simple command line that converts a binary file to an array of bytes, defined at a source of another language, so you can embed it into your program.

Currently supported output languages:

* C
* C (shellcode)
* Pascal
* Python
* Rust


<a name="overview"></a>
## Overview

**bin2src** reads a binary file (.jpg, .wav, .mp3, etc.) and generate a source file with the binary
data embeded into it as a byte array.  

Sometimes, maybe you don't want to distribute a binary file inside your program's package and 
users can access it directly.  

With **bin2src** you can embed it inside the executable and read the bytes direct from memory.   

Keep in mind that it's always possible for an "advanced" user extract the file, even inside the 
executable.

### Give a Star! :star:
If you like this project and find it useful, please give it a star. I'll appreciate very much! 
Thanks!

<a name="usage"></a>
## Usage

<pre>
bin2src < -l LANG | --out-lang LANG > [ OPTIONS ] < FILE >

LANG and FILE are required.

Options:

        -l, --out-language LANG         specify the language, where LANG={c|cshell|pascal|python|rust}

        -d, --out-dir PATH              specify where to output source(s) file(s);
                                        if not specified, generate in current directory

        -f, --out-file OUTFILE          specify the output file(s) name (* without extension *);
                                        if not specified, output file(s) will have the same name
                                        of input file (without extra dots).

        -h, --hex                       output bytes in hexadecimal (for C shellcode this flag has
                                        diferent behaviors. See the Github site for more information)

Currently supported languages:

  - C
  - C for shellcode
  - Pascal
  - Python
  - Rust	
</pre>

## Examples

Suppose you have an image `myimage.jpg`:
<br>
<br>

<a name="example1"></a>
**Example 1:**

```
bin2src --out-language pascal --out-dir "X:\My Projects\project01" --out-file image01 myimage.jpg
```

<sub>Windows paths with spaces needs quotation marks</sub>

will create the file `...\image01.pas` with bytes in decimal format: `[210, 0, ...]`.
<br>
<br>

<a name="example2"></a>
**Example 2:**

```
bin2src -l c -d "X:\My Projects\project02" -f image01 -h myimage.jpg
```

will create the files (with bytes in hexadecimal: `[0x10, 0xfa, ...]`):

* `...\image01.h`
* `...\image01.c`

<br>

<a name="example3"></a>
**Example 3:**

```
bin2src --out-language python myimage.jpg
```

will create the file "myimage.py" at the current directory.
<br>
<br>
Check the [examples directory][3] for some practical uses of bin2src.

## Atention

* Beware with the **file size** that you'll embed in your code!!!

  Verify if it's accepted by your O.S., compiler, language standards, memory at runtime, etc.

* if the file has more dots, in addition to the dot that separates the extension name and
  you don't use the `--out-file` or `-f` command line option, the output file name will 
  be the first name before the first dot. Example (generating a 'y' file):
  
  `abc.def.ghi.x` => `abc.y`
  
* The behavior of the option `--hex` or `-h` for C shellcode is different than the other 
  languages. Without this flag, it will generate an array of `unsigned char` bytes, but with
  the hexadecimal flag, it will embed the bytes as a string (`char *`).
  
* If you'll generate C shellcode as string, make sure that the binary does not contais null
  bytes ("\x00") or don't use string functions like `strlen`. This may break your code
  and could cause exceptions (access violations, etc.).

* All the tests was made (until now) with Windows 10 Pro (2004) and to execute the alpha release
  maybe you have to install the latest [MSVC runtime][4].
  
* There are a lot of things to organize and improve the project. Please, check the [TODO][5] list.

<a name="license"></a>
## License

Developed by Alexandre Gomiero de Oliveira under the [GPL-3.0 License][1].

Any code generated by **bin2src** are under [MIT License][2].

Please contact me if you need a different license.

If you'll use the tool to develop commercial products, please, consider make a donation 
to help me with future projects. :smiley: :thumbsup: :pray:

[1]: ./LICENSE
[2]: ./LICENSE-GENERATED
[3]: ./examples
[4]: https://support.microsoft.com/en-us/help/2977003/the-latest-supported-visual-c-downloads
[5]: ./TODO.md
