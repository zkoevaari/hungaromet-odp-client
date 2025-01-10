TODO
====

## Input ##

- download latest zip
- download older zips?
- zip
- csv:
    - raw and condensed
    - delimiter: default, auto (header), char
    - column filtering:
- csv STDIN
- zip STDIN?


## Output ##

- zip write (when input is zip?)
- zip STDOUT?
- csv write
- csv append
- csv STDOUT
- csv formats:
    - raw (default)
    - condensed
    - delimiter: default, auto (header), char
- database
- tee: db and STDOUT
- validation report
- help print column lists


## Filter ##

- rows
    - station_name (with `like` syntax)
    - station_number
    - comma separated list
    - invert
    - distinct
- columns
    - all
    - reduced: no EOR, Q
    - noinfo
    - invert
    - column index, name
    - comma separated list with ranges


## Database ##

- check if `data` table is compatible with selected output columns
- create tables if necessary
    - option to emit script instead (or on error)


## Misc ##

- config file
- daemon
- Anything other than 10-minute latest?
