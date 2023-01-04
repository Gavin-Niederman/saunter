# Saunter

A library for tick based game engines.

<!--write the ## How to use Saunter  section-->

**What is a tick based game engine?**

A tick based game engine runs all of it's calculations at fixed intervals. This is in contrast to most modern game engines which run their calculations every frame. Tick based game engines are used because it allows for calculations to be run on a different thread than the rendering thread. allowing for the game to render at a consistent speed even if the rendering thread is slow, and for the rendering thread to run quickly even if the calculations are slow.

When multiple frames happen per tick interpolation is used to make the game appear to run smoothly. This is done by storing the tick data for the previous tick, and the most recent tick and interpolating between the two to get what the renderer should show.
