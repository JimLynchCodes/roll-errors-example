# roll-errors-example

in the `my-new-proj` folder, run:
```
anchor build
```

<br/>


note: if it doesn't work, you may need to update things:
```
agave-install update
rustup update
cargo update
anchor clean
anchor build

```

<br/>

## Goal: Get Place Bet Function Compiling

The commented out code shows a function and structs to go along with it. The problem is that when I enable this code I see many errors such as the ones below. The goal is to get it compiling clean and working properly with this `place_bet` function

```
^^^^^^^^^^ the trait `anchor_lang::Accounts<'_, _>` is not implemented for `PlaceBet<'_>`

pub fn place_bet(ctx: Context<PlaceBet>, guess: u8, amount: u64) -> Result<()> {
   |                           ^^^^^^^^^^^^^^^^^ unsatisfied trait bound

 ^^^^^^^^^^ the trait `anchor_lang::Accounts<'_, _>` is not implemented for `PlaceBet<'_>`
   |


 #[program]
   | ^^^^^^^^^^ the trait `Bumps` is not implemented for `PlaceBet<'_>`

error[E0599]: no function or associated item named `try_accounts` found for struct `PlaceBet` in the current scope
   --> programs/my-new-prog/src/lib.rs:37:1
    |
37  | #[program]
    | ^^^^^^^^^^ function or associated item not found in `PlaceBet<'_>`
...


   --> programs/my-new-prog/src/lib.rs:289:21
    |
289 | pub struct PlaceBet<'info> {
    |                     ^^^^^ unused lifetime parameter
    |
```