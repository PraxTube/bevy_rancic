# Bevy Rancic

Adds highly opionated common functionalities for 2D top down games in Bevy.

## TODO

So I was pondering the other day and realized that we may not need this complicated `YSort(Child)` stuff. Instead of using regular Parent/Child pattern we can use Sibling structure. Concrete meaning:

Instead of

```
Player (Parent)
----> Shadow (Child)
```

we use

```
Player (Independent)
Shadow (Independent)
```

Then you just store the player `Entity` inside the shadow component and in a specific system you simply update the YSort and transform based on the target (player in this case). One thing I am not too sure about here are the following two things:

1. We would need to search all possible target entities, which would be either impossible or quite a big query. The alternative would be to write this update systme for every shadow (or other pseudo child entity) individually, this would allow you to specify the target query much more (making it a lot smaller) but it would also be a pain in the ass and the bevy user would need to implement it (instead of having it here in this crate `bevy_rancic`).
1. I actually don't know of this is feasable at all. Depending on when the transform gets updated or the rapier velocity update it might be impossible to have this in a way that makes sense (compated to the parent/child which works in its current way).
