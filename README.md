# Album card generator for Qobuz
This small tool generates a web page fro printing album cards for my [Qobuz rfid player](https://github.com/SofusA/qobuz-player/wiki/RFID-player).

## Usage
`cargo run -- --username <USERNAME> --password <PASSWORD> --album-ids <SPACE SEPARATED ALBUM IDS>`
album ids can be retrieved from the url from Qobuz: `https://play.qobuz.com/album/<ALBUM ID>`

Then open `localhost:3000` and print the page
