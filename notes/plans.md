# Next steps

## GUI
- Decide on a design for the general purpose DPS calc/simulator
- Write the code for the chosen design
- Investigate web worker error (figure out if it's exclusive to iOS and what causes it)
- Deploy the website—maybe when there are a few good simulators done in addition to the general calc

## Backend (general)
- Extract the spec strategy logic from the single-way simulation to make it generalizable to all simulations
- Add inventory setup/tracking to enable simulating finite-length trips
- Fully flesh out the food system
- Remove `main.rs` and move the code there to `/examples/`
- Improve documentation
- Improve test coverage
    - More unit tests in general
    - Add integration tests for new edge cases and weapons/effects that have come into the game since writing the original ones

## Simulations
### In progress
- Vardorvis
    - Nail down the timing and consistency of Vardorvis dashing back and forth
    - Implement specs
    - Do a bit more research on exactly how the defense/strength scaling work and what does and doesn't trigger it
- CG: I think this is done
- Graardor
    - Do more VOD review to determine whether the minion timings are actually consistent
    - If not, either figure out a reasonable average behavior, or start working on adding a position/pathing system
    - Research all other solo methods and add them if they're reasonably popular

### Next up
- Vorkath
    - Do some VOD review to figure out the cooldown on the pink dragonfire attack
    - Nail down exactly how eat timing works during the zombified spawn special
    - Chart out the full flow of the fight with exact timings
- Zulrah
    - Review and take notes on Simetra's Zulrah sim, and verify timings against VODs/testing
    - Chart out the fight with all timings
    - Do some testing on how subsequent rotations work on long kills
    - Research all of the (good) Zulrah tech
    - Pathing/LOS system may also come in handy for simulating snakelings here
    - Come up with a system for allowing users to configure "attack charts" where they can mix and match attacks with different weapons for each phase
- Muspah
    - Review my notes and VODs from earlier
    - Figure out exactly how smite skip works and how to implement it in the sim
    - Chart out the fight with all timings

### Other feasiblen solo simulations
- Sarachnis
- Moons (not counting time between bosses)
- KQ
- Kree
- Zilyana (again, pathing system would help)
- K'ril (see above)
- Solo Huey
- KBD
- Most wildy bosses
- Solo Royal Titans
- Yama (at least until I get to the really complicated melee P3 methods)
- Duke
- Leviathan
- Whisperer
- Hespori
- Sire
- GGs
- Kraken
- Cerb
- Araxxor
- Thermy
- Hydra
- Zuk
- Nightmare (would need to actually learn about this boss first)

### Raids (solo, for now)
- ToA
    - Ba-Ba should be pretty easy
    - Kephri is tricky because of swarming and killing scarabs (and also having to account for more overlords)
    - Akkha could be fairly simple if I can assume no tick loss on the enrage phase
    - Zebak should be very simple
    - Obelisk is straightforward
    - P2 formula is now known, so that can also work - may need an attack chart like Zulrah/Verzik P1 for the core
    - P3 and P4 are dead simple, assuming perfect play
    - Overall, if I can figure out how to deal with Kephri, this feels achievable for solos
- CoX
    - Similar to ToA in a lot of regards for solos, though I don't think anything is as complicated as Kephri swarming
    - Main challenge will be including all of the different tech you could use
    - There's also Synderis' CM sim that I could use as a reference/comparison when researching mechanics
- ToB
    - Does not seem feasible to simulate the entire thing, even solo (Nylos would be a nightmare)
    - If/when I add group simulations, this will become more feasible
    - Would require a huge deep dive into tech and would probably be massively bloated with configuration options in the GUI

### Demi-bosses or other monsters that have mechanics
- Demomnic gorillas
- Tormented demons
- Most types of dragons and wyverns

# Long-term goals
- Add multiplayer simulation capability for group bossing
- Add position tracking and a pathing engine for simulations where NPC movements and attack ranges need to be tracked (e.g., GWD kiting/red-x methods)

# If I'm bored
- Implement loot rolls/drop simulators for fun
- Simulate continuous trips with banking, etc. to simulate kph and/or slayer xp/hr
- Add a gear optimizer (probably not going to happen, but would be cool and is clearly in high demand)
