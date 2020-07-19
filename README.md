The intention of this small tool is to read Vice City savegames
and write them to a .yaml file. You can modify that yaml file
and then have this tool apply the changes you made to the
original savegame file.

This is currently in development and I don't know how far I'll
push it. My personal motivation is to be able to fiddle with
the car generators. For example, I placed an apaci on the roof
of the estate :-) (Though I did this with a hex editor.)

What currently works is conversion of the car gen section
of a savegame to a .yaml file. Example:

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
