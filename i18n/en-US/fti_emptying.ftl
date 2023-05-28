# Messages
CONTAINER-emptied-at-NOW-next-emptying-at-NEXT =
    {$container}: Emptied {$now}, next emptying {$next}
CONTAINER-just-emptied =
    {$container} has just been emptied

# Times in the past
now = now
SECONDS-seconds-ago =
    {$seconds ->
        [1] a second
       *[other] {$seconds} seconds
    } ago
MINUTES-minutes-ago =
    {$minutes ->
        [1] a minute
       *[other] {$minutes} minutes
    } ago
HOURS-hours-ago =
    {$hours ->
        [1] an hour
       *[other] {$hours} hours
    } ago
DAYS-days-ago =
    {$days ->
        [1] a day
       *[other] {$days} days
    } ago
a-really-long-time-ago = A really long time ago

# Times in the future
DAYS-IN-THE-FUTURE =
    {$days ->
        [0] today
        [1] tomorrow
        [2] the day after tomorrow
       *[other] in {$days} days
    }
