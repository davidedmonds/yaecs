# yaecs

Yet Another Entity-Component System. Fed up with the vast variety of ECS
available in Rust crates, I decided "me too" and dived in with this effort.

## Current state

At present, this is a deliberately simple and naive implementation, as my poor
little brain hurt when I looked at other awesome systems such as [ecs](https://github.com/HeroesGrave/ecs-rs)
and [specs](https://github.com/slide-rs/specs). Hopefully over time as my code
using this matures I'll spend some time optimizing what's going on in here and
trying to remove any bottlenecks, though this will likely only be done once I've
profiled the application.
