# Zvenira
> Chess pairing software

---

Zvenira is a pairing system that includes the models or algorithms for matches and tiebreaks mostly used across official and unofficial tournaments across de world.

---

The pairing models are:
- Suize System
- Round Robin
- Teams 
- Knockout

---

Tiebreakers (in the official order of FIDE):
1. Points
2. Buchholz cut 1.
3. Buchholz cut 2.
4. Sonneborn-Berger

--- 

The program will keep a local record per tournament with the next basic data:
- player
- Country
- Federation
- Rating

each player while during the tournament will have a history of games with the basic data of:
- Acumulated points WHILE the match
- Round
- Color side
- Oponent
- Result (0, 0.5, 1, 1 with bye)

---

We will implement a sign mechanism with public and private keys to share and validate the results of games across all the world
