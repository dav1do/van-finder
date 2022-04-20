# van-finder
Searching for a new van...

This is not productionized in any sense and is purely a personal utility at this point. Note that it currently only supports one site (https://thevancamper.com/campervans-for-sale) and the query is [hardcoded](https://github.com/dav1do/van-finder/blob/e4b3c58761ab028220ed2be04ac209c3eee3ccdd/van-finder/src/sites/van_camper.rs#L16).

## Running

If you do find yourself interested in running it, here are the steps: 

1. Update your .env to match the .env.sample with real values. you need to allow 'less secure apps' so you can avoid 2factor auth with gmail. see [this article](https://webewizard.com/2019/09/17/Using-Lettre-With-Gmail/)
2. `cargo run` 

The binary will spawn a task that loops on a 24 hour interval by default. It will check the highwater file (empty first time) and then email you a list of all the vans that match the query. On next run (i.e. after 24 hours), it will repeat the query and email you all the new posts that have shown up since the previous highwater mark van. This logic is not very super smart, and I don't think it handles the highwater van getting taken down (should use created timestamps as a fallback/secondary check). 

If you don't like it running in the background, you can stop it and just run it manually when you want to see if there are any updates.
