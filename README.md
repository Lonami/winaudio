# winaudio

Enables audio playback of waveform audio in Windows (play sounds from wav files).

## Description

This crate serves as an abstraction over the audio functions provided by the
[Windows Multimedia API]. In particular, it enables its users to enumerate the output devices,
list their capabilities, and open them for playback. Basically, it lets you play sound files
in Windows:

```rust
use winaudio::wave::Player;

fn main() {
    let mut player = Player::from_file("test.wav").unwrap();
    player.play().unwrap();
}
```

If you're missing a certain function from the [mmeapi.h header], feel free to open an issue
or send a pull request to the project to add it. This initial version doesn't have methods to
set the pitch or playback rate for example, but they can trivially be added if needed.

## License

All the libraries contained in this repository are licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE] or
  http://www.apache.org/licenses/LICENSE-2.0)

* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Windows Multimedia API]: https://docs.microsoft.com/en-us/windows/win32/api/_multimedia/
[mmeapi.h header]: https://docs.microsoft.com/en-us/windows/win32/api/mmeapi/
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
