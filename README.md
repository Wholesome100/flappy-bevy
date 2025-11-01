# Flappy Bevy

This is a small, basic project to get familiar with the Bevy game engine. Bevy itself is written in Rust and utilizes an Entity Component System (ECS) architecture
that allows for innate multithreading. While the engine is young, I wanted to get a glimpse into what development with it was like.

## Overview

Flappy Bevy is a clone in the vein of the original Flappy Bird. You control a "bird" and are attempting to fly through gaps in various "pipes" to earn points. The game ends
when the bird collides with a pipe. The assets for the game are all basic engine shapes, as the programming side of this project was the main focus. The project utilizes the Avian2D library to handle the game physics and collision events.

## Takeaways

Bevy is a very interesting engine and is doing a lot of things right. The ECS is intuitive and enables multithreading by nature of its design. However, without an editor, 
iteration time is slower than other engines like Godot, although there is a Bevy Editor currently in the works. Additionally, the lack of established documentation made 
development harder, as the only definitive sources of information were the Bevy examples, docs.rs, and the Bevy Cheatbook.

Still, I believe this engine is very promising, and it is something I look forward to revisiting in the future with the exciting changes to the UI system and the already 
mentioned editor.

## Running the Project

To run this project, you will need to have the Rust language installed on your computer. From there, clone the repository and cd into it. Run the command `cargo run` 
inside the directory and after a short compile time, the game will open on your desktop.
