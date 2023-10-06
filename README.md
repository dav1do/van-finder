# van-finder

Searching for a new van...

This is not productionized in any sense and is purely a personal utility at this point. Note that it currently only supports one site (https://thevancamper.com/campervans-for-sale) and the query is [hardcoded](https://github.com/dav1do/van-finder/blob/e4b3c58761ab028220ed2be04ac209c3eee3ccdd/van-finder/src/sites/van_camper.rs#L16).

## Running

If you do find yourself interested in running it, here are the steps: 

1. Update your .env to match the .env.sample with real values. you need to allow 'less secure apps' so you can avoid 2factor auth with gmail. See "app passwords" in google (old linked article with info is a 404 now)
2. `cargo run`

The binary will spawn a task that loops on a 24 hour interval by default. It will check the highwater file (empty first time) and then email you a list of all the vans that match the query. On next run (i.e. after 24 hours), it will repeat the query and email you all the new posts that have shown up since the previous highwater mark van. This logic is not very super smart, and I don't think it handles the highwater van getting taken down (should use created timestamps as a fallback/secondary check). 

If you don't like it running in the background, you can stop it and just run it manually when you want to see if there are any updates.

## TODO list

Not sure why I made it a workspace.. so maybe get rid of that first. And if it were worthwhile, one could do a lot of things to make this more useful. here are some ideas:

- support email that isn't hacky gmail (e.g. sendgrid)
- use a sqlite/real DB instead of a file for highwater mark
- better error handling
  - if the email fails, it never retries. next time it thinks it succeeded.
- support multiple sites and not fixed queries. run as lambda and allow sign up via API.
- parallelize things
  - right now, it's a big main loop and then sleep. could run it more like an api, main loop and tasks are injected
    - possibly actor style with channel<ReqEnum(ReqType)> and then delegation to each engine (van_camper, etc that impl some trait)