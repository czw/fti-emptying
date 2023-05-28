# Messages
CONTAINER-emptied-at-NOW-next-emptying-at-NEXT =
    {$container}: Tömdes {$now}, nästa sker {$next}
CONTAINER-just-emptied =
    {$container} har tömts

# Times in the past
now = nu
SECONDS-seconds-ago =
    {$seconds ->
        [1] en sekund
       *[other] {$seconds} sekunder
    } sedan
MINUTES-minutes-ago =
    {$minutes ->
        [1] en minut
       *[other] {$minutes} sekunder
    } sedan
HOURS-hours-ago =
    {$hours ->
        [1] en timme
       *[other] {$hours} timmar
    } sedan
DAYS-days-ago =
    {$days ->
        [1] igår
        [2] i förrgår
       *[other] {$days} dagar sedan
    }
a-really-long-time-ago = Ruskigt länge sedan

# Times in the future
DAYS-IN-THE-FUTURE =
    {$days ->
        [0] idag
        [1] i morgon
        [2] i övermorgon
       *[other] om {$days} dagar
    }
