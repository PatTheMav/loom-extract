Loom CDDA Converter
===================

This is a single-purpose command-line tool: To convert the `CDDA.SOU` file of the FM Towns-based VGA version of Loom (as sold on e.g. Steam or GOG) into a WAV file for further audio processing.

# The Format

LOOM's VGA version uses a single CD audio track that contains all dialog as well as musical cues much like a radio play which the game seeks to upon corresponding events in-game.

The version distributed on Steam and GOG contains a single file called `CDDA.SOU` that contains this single track, but although its name suggests this to be a PCM-based CD audio (e.g. a simple `AIFF` or `WAV` file) it actually uses a bit-shifted 8-bit format:

| Byte(s) | Description |
| ------- | ----------- |
| 1-800   | Unknown data, considered "garbage" |
| 801     | Bit shift value |
| 802-1978 | 1176 bytes of unshifted audio |
| 1979 | Bit shift value |
| 1980-3156 | 1176 bytes of unshifted audio |

# The Conversion

To retrieve the 16-bit audio encoded in the file, each block of 1177 bytes needs to be processed in the same way:

1. Read the first byte - it contains the shift values
2. Shift the upper 4 bits down to yield the shift value for the _left_ channel (`byte_value >> 4`)
3. Mask the lower 4 bits to yield the shift value for the _right_ channel (`byte_value & 0x0F`)
4. Read the second byte and treat it as a _signed_ 8-bit value
5. Convert this value into a _signed_ 16-bit value
6. Left-shift this value by the amount of bits from step #2
7. Read the third byte and repeat step #4
8. Repeat step #5
9. Repeat step #6, but this time use the amount of bits from step #3
10. Continue until reaching a multiple of 1177 bytes, at which point repeat from step #1

## Explanation

The `CDDA.SOU` file contains 16-bit stereo audio samples, but encoded to 8-bit values with their lower 8-bits removed. As such this compression is _not_ lossless, as amplitude detail of the first 8 bits is lost - with 15 bits of amplitude in either direction, this comes down to a maximum accuracy loss of 0.7% (and that's at peak amplitude - the lower the volume, the more detail is retained).

Given two bytes (`10011100` for the left channel, and `11100001` for the right channel) and the bit shift value `01000100`, this would evaluate as follows:

```
uint8 bit_shift = 68;                               // 0100 0100
uint8 left_channel_shift = bit_shift >> 4;          // 0100 => 4
uint8 right_channel_shift = bit_shift & 0x0F;       // 0100 => 4

int8 left_channel_enc = -28                         // 1001 1100 => -1 * 001 1100 => -28
int8 right_channel_enc = -113                       // 1110 0001 => -1 * 111 0001 => -113

int16 left_channel_dec = -448                      // 1000 0001 1100 0000 => -1 * 000 0001 1100 0000 => -448
int16 right_channel_dec = -1808                     // 1000 0111 0001 0000 => -1 * 000 0111 0001 0000 => -1808
```

These decoded 16-bit values can now be written to a file with a proper header for 16-bit little-endian PCM audio and be played as-is.

# A Different Kind Of Hello World

This is the first bit of Rust I've ever written, so code style inadequacies are to be expected 🥴..

# Acknowledgements

* Format discovered by user "apprentice_fu" on the [official ScummVM forums](https://forums.scummvm.org/viewtopic.php?p=64394#p64394)
* Python 2-based extractor by Github user "symm" named [LAAExtract](https://github.com/symm/LAAExtract)
