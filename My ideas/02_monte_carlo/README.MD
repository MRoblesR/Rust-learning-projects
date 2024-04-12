# Monty Hall Problem Monte Carlo Simulation
This program that uses a method called [Monte Carlo](https://brilliant.org/wiki/monte-carlo/) simulation to understand the [Monty Hall problem](https://brilliant.org/wiki/monty-hall-problem/).

## The TV game
Imagine a game with three doors. Behind one door is a special prize, like a car, and behind the other two doors are less exciting prizes (for me), like goats.

Here's how the game works:

1. You pick a door, hoping the car is behind it.
2. The game host opens one of the other doors, revealing a goat.
3. Now, you have a choice: stick with your first choice or switch to the other unopened door.
4. If the car is behind your chosen door in the end, you win it.

## The Statistical Problem
People used to debate whether it's better to stick with your first choice or switch doors in the end. Some thought it didn't matter, and both choices had a 50% chance of winning. But math and simulations proved otherwise. Switching doors actually gives you about a 66% chance of winning.

## What's a [Monte Carlo Simulation](https://brilliant.org/wiki/monte-carlo/)?
It's a fancy name for a simple idea. We're going to play this game many times, randomly picking doors each time. By doing this lots of times, we'll see which strategy—sticking or switching—gives us more wins.

Since I'm not a Rust expert, this simulation is a good way for me to understand and start working with simulations too.

## The results

The simulation confirmed it: changing the door actually makes it more likely to win. We saw that switching doors doubled our chances of winning compared to sticking with our initial choice.







