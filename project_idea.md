# Final project idea description

## 1. Motivation

https://sio2.mimuw.edu.pl/c/oi32-1/p/kas/

https://sio2.mimuw.edu.pl/c/pa-2024-1/p/dzi/

In the Polish Olympiad in Informatics and other conetsts there are a lot of problems including communication with library. 

The concept of this project is to create an app with puzzle games similar to those in the links above.

Logic of these games is quite simple. Most of them are 'guess the number' style.
Thinking of the game 'casino' player has one of the following actions:

- ask(y) - user asks server for an value f(x, y), where x is the secret value to guess and f is some function
- guess(y) - user answers if the secret value is y, if x=y they get a point
- reroll() - user asks to reroll the secret value, which is sampled uniformly from some interval

## 2. Desired work plan

First part:

- Implement a single player game:
- [ ] system of points, hearts, timer
- [ ] different game modes (two links above & some others)
- [?] think of 'break the cypher', where a goal is to find the function f
- [ ] graphic interface
- [?] cover code with tests (UT & integration)

Second part:

- Introduce web version:
- [ ] allow the online version of the game
- [ ] server handles more than one player
- [ ] user creates the account and their scores are saved (database?)
- [ ] live ranking of the best scores

## 3. Why it could be useful?

There are a lot of similar problems in contests. This app can show contestants the visualization of their job, demonstrating that they actually create a bot for a real game. Additionally, it could be easier for them to test their bots on a ready server.

This project could also be a good fit for the Polish Olympiad in Informatics.

---

Author: Łukasz Skiba