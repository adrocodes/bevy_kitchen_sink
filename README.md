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