# Vercettus command line utility

## Intention

The intention of this small tool is to read Vice City savegames
and write them to a .yaml file. You can modify that yaml file
and then have this tool apply the changes you made to the
original savegame file.

This is currently in development and I don't know how far I'll push
it. So far, you can edit the *car generator* section and apply your
changes to the original savegame. Car generators are responsible for
respawning cars, helicopters and boats at certain predefined locations
in the game.

For example, I placed an apaci on the roof of the estate :-)
(Helicopters seem to count as cars, for all data structure intents and
purposes)

You can modify the existing car generators and *add _a few_ new ones*,
until the few extra bytes available in the savegame file run out. You
can change the locked and alarm flags and set the colors of the cars.
You can also fiddle with the coordinates.

## Steps

### BACKUP your savegames!

### Export the savegame to yaml:

    vercettus -c "C:\path\to\your\savegame" -o sg.yaml

### edit sg.yaml

### Apply your changes:

    vercettis -i sg.yaml -o "C:\path\to\your\savegame"

## Couple Notes

It is recommended that you apply the .yaml file to the same savegame
file you generated it from. While it is possible to apply it to a
different savegame file, this may cause unanticipated results. For
example, each car generator has a "next generation time" timestamp
stored on it. Now, if you were to generate a .yaml file from a pretty
advanced game state and to import it to a lesser one, your in-game
timers may be much smaller than the ones set on the cars, leading to
the car generators to not work until you wait a lot of time. You have
been warned.

## More notes

* Patches welcome! :-) I'd also like to hear from happy users.

* This currently only works with the PC steam edition of the game.

## Even more notes

This utility would not have been possible without the stuff found at
various websites, most notably https://gtamods.com/wiki/Saves_(GTA_VC)

## Excerpt of a yaml file:

    ---
    - []
    - preamble:
        last_mission_passed: "Hog Tied"
        w_year: 2020
        w_month: 7
        w_day_of_week: 2
        [...]
      car_generators:
        process_counter: 1
        generate_even_if_player_is_close_counter: 0
        active_car_generators: 131
        generators:
          - vehicle: Deluxo
            coordinates:
              x: -1022.5999755859375
              y: -868.5999755859375
              z: 12.21500015258789
            heading: 175.0
            primary_color: 65535
            secondary_color: 65535
            force_spawn: true
            alarm: false
            lock: false
            min_delay: 0
            max_delay: 10000
            game_timer_when_car_is_generated: 61
            vehicle_index: -1
            is_on: false
            recently_stolen: false
          [...]
