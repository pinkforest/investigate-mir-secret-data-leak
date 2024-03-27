# experiments

$ cargo run --release --bin eq_sodium_terrible_imitation -- --continuous eq_sodium_terrible_imitation
```
bench eq_sodium_terrible_imitation seeded with 0x89e9eeffd6506f0b
bench eq_sodium_terrible_imitation ... : n == +0.024M, max t = +1.33052, max tau = +0.00864, (5/tau)^2 = 335043
bench eq_sodium_terrible_imitation ... : n == +0.033M, max t = +1.40642, max tau = +0.00774, (5/tau)^2 = 417475
bench eq_sodium_terrible_imitation ... : n == +0.263M, max t = -2.26579, max tau = -0.00442, (5/tau)^2 = 1281015
```

$ cargo run --release --bin eq_vartime -- --continuous eq_vartime
```
bench eq_vartime_yes seeded with 0x3fe0fa7cdd0ce8e8
bench eq_vartime_yes ... : n == +0.017M, max t = +303.78975, max tau = +2.29920, (5/tau)^2 = 4
bench eq_vartime_yes ... : n == +0.035M, max t = +423.53500, max tau = +2.26089, (5/tau)^2 = 4
```
