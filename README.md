# Memory Scanner
This project was for a security tool development course. I decided to use this course to learn Rust and the basics of memory scanning in Windows. This meant utilizing the Win32 API and scanning processes memory. The idea is that it would be similar to the cheat engine for scanning for specific values. At the time of writing, this project was one of the only github projects that used Microsoft's Rust library for Win32. Other projects used the unofficial Win32 library which, in hindsight, may have been a better choice due to how new the Microsoft library is and the lack of examples it currently has.

## A few highlights of this project
- Modular design so that parts of it can be imported into other projects.
- Signed and Unsigned integer, float, with both 32-bit and 64-bit value scanning
- Command line functions to scan for a value without the need to program
- Server API using Rocket.rs for use in a web application. 


 
