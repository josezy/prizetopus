# Prizetopus

Solana program to distribute the prize pool of a tournament (ie. poker)

## 1. Actors

- Tournament authority (TO)
- Participants (Pn)

## 2. Flow of the game in the contract

1. Create the tournament
  - rules
    - distribution of the prize pool
    - buy-in
    - number of participants
    - fixed date timeout
    - password?
    - max_voters - number of top players left to vote on changing the prize distribution
  - wallet for the prize pool PDA

2. Participants register
  - pass wallet
  - pass PDA for player balance (which holds chips)

3. TO starts the tournament
  - joining locked after that

4. TO sends an update every time after player gets eliminated 

5. Once n_players <= max_voters, player can raise a voting proposal to change the prize distribution
  - If all sign before the next update, the distribution is changed
  - If update comes before all participants join, proposal is invalid

6. If only 1 player is left, or the proposal is accepted, the tournament ends
  - Prize is split according to the distribution rules

7. If the TO stops updating the game, the players can redeem their prize according to the last known state of the game