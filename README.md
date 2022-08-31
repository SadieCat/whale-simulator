# 🐋 Whale Simulator 🐋

Whale Simulator is a terminal game written in Rust where you are a whale and you have to avoid pesky fishing boats whilst snacking on delicious krill.

## Why?

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">concept: a whale simulator where you’re just trying to keep fed while avoiding the boats with harpoons<br><br>score counter is your krill/death ratio</p>&mdash; Luna 💙 (@lunasorcery) <a href="https://twitter.com/lunasorcery/status/1481045191964385283?ref_src=twsrc%5Etfw">January 11, 2022</a></blockquote>

## Warnings

- Whale Simulator was written for Rust 1.57.0. It may not build under older versions of Rust.
- Whale Simulator has only been tested in Gnome Terminal. It might lag or not work at all on other terminals. Try passing lower values in `--tick-rate` if it lags.
- Whale Simulator lacks a unified entity management system so its logic is a bit spaghetti.
- Whale Simulator does not handle window resize events. Don't resize your terminal or it will break.
- Whale Simulator uses `.unwrap()` everywhere when writing to stdout which is horrible but it is unlikely to fail.

## Contributing

Please don't. This isn't a serious project. It was written in one night as a joke. I don't want to actually maintain this.

## Credits

Thanks to Luna (@lunasorcery) for the idea.
