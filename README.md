# Bevy Kitchen Sink

A collection of random plugins and small games to learn and test out Bevy related features.

> Warning: the code in here will most likely be quite rough as it is a playing ground, be warned if you want to copy & paste any of the code :D

If I build something interesting, I'd extract it into a separate repo that you'll be able to use in your projects. Check out the `Cargo.toml` for those types of packages.

## Command Defender

Tracks user input and stores it in a `CommandInput` resource. It will only accept alphabetic, alphanumeric & `:` at this point. This logic will be used in a game where the player controls the game through putting in commands on a grid. For example `B2:C3`. Move from `B2` to `C3`.

## Select Area

Allows the user to click and drag a rectangle to select entities in that region.

## Pancam

A copy of this plugin - https://github.com/johanhelsing/bevy_pancam. Used to learn how it works and make my own changes for customisation.

## Solar System

A really basic Solar System simulation.

### Bounds

A simple square bounds that can be used to detect if the mouse is over a given item.

**Only works with Rectangle/Square shapes**

### Mouse Over

Demo of using **Bounds** to detect mouse over state.

### Examples - Random Squares

Generates a square of tiles that are randomly assigned a tile based on a random 0-1 value. Just uses the `rand` package to generate the values.

### Examples - Perlin Squares

A copy of **Random Squares** but the `rand` package has been replaced with `noise` to generate the square using Perlin Noise.

### Examples - Doodle Demigod

A minimal clone of the game "Doodle God", where instead of combining elements to make new things, you combine land tiles to unlock decorations. There are 7 tiles to unlock.

### Examples - Wave Collapse

Using a very scuffed "Wave Collapse" it will generate a random path network. Some paths just kinda end, plan is to refine this later to be a bit more accurate and interesting.