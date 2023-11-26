# Morse

`morse` is a simple program to convert between a limited subset of ASCII and a binary
representation of morse code.


## Usage

```
A morse code translator

Usage: morse [OPTIONS] <DIRECTION>

Arguments:
  <DIRECTION>  [possible values: text-to-morse, morse-to-text]

Options:
  -s, --src <SRC>  The source of the translation
  -d, --dst <DST>  The destination of the translation
  -h, --help       Print help
  -V, --version    Print version
```

## Encoding

`morse` is able to encode and decode all letters `A`-`Z`, `0`-`9` and the punctuation
characters `:`, `.`, `?`, `/`, `-`, `(`, `)` and ` ` (space).

True to morse code, each character is encoded as a series of dits (short) and dahs (long),
written `.` and `-` in the rest of this document. This is the translation table used for
each letter.

```
'A':'.-',  'B':'-...',  'C':'-.-.',  'D':'-..',  'E':'.',  'F':'..-.'
'G':'--.',  'H':'....',  'I':'..',  'J':'.---',  'K':'-.-',  'L':'.-..'
'M':'--',  'N':'-.',  'O':'---',  'P':'.--.',  'Q':'--.-',  'R':'.-.'
'S':'...',  'T':'-',  'U':'..-',  'V':'...-',  'W':'.--',  'X':'-..-'
'Y':'-.--',  'Z':'--..'

'1':'.----'
'2':'..---'
'3':'...--'
'4':'....-'
'5':'.....'
'6':'-....'
'7':'--...'
'8':'---..'
'9':'----.'
'0':'-----'

':'--..--'
'.':'.-.-.-'
'?':'..--..'
'/':'-..-.'
'-':'-....-'
'(':'-.--.'
')':'-.--.-'
```

## Binary Encoding

A string is encoded as a series of characters, where each character is a sequence of dits
and dahs with a character end symbol ending each character. Each dit is encoded as `10`,
representing a short pulse and then a pause. Each dah is encoded as `11`, representing a
long pulse. Each character is ended with an extra long pause: `00`.

That meas that the character `A` (`.-`) is written as `101100`. The word `HI` is encoded as:

```
10101010 00 1010 00 00
^H------    ^I--    ^- word end
```
