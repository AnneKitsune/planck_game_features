Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

# Planck ECS Game Features
Layer 2 Crate integrating the game_features library with the Planck ECS library.

Provides systems for updating the different game_features structs.
Also provides a system bundle to easily integrate into your games.

# Usage
Add the following to you Cargo.toml file:
```
planck_game_features = "*"
```

Use it like so:
```rust
use planck_game_features::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Stats {}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Effectors {}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Skills {}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Items {}

fn main() {
    let mut dispatcher = DispatcherBuilder::default();
    dispatcher = GameFeaturesSystemBundle::<Stats, Effectors, Skills, Items>::insert(dispatcher);
}
```

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

