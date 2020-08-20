Flower Pi
=========

A raspberry pi powered flower that shows the current pollen count.

Features
--------

- Scrapes the Met Office for UK regional pollen count
- An LED clock, the background for which represents the pollen count (red = high, yellow = medium, green = low)
- A sensor that turns the LEDs on for a few seconds when it notices movement
- Error reporting using IFTTT (keyed by an environment variable)
- Signal handling to turn off all the lights if the program is asked by the OS to stop

Missing features:
-----------------

- Tests 😭

Physical Parts:
---------------

Controlled by the app:
- Raspberry Pi Zero W
- RasPiO InsPiRing Driver Board
- RasPiO InsPiRing Circle
- PIR sensor, connected to GPIO 17, GND and 3.3V

Additionally, by editing `/boot/config`:
- a button (with resistor) to turn the device on/off by connecting GPIO3 to GND.
- a power led (with resistor) connected to GPIO 13 and GND
