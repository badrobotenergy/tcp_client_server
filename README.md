# Rust Client Server Echo App

First go at trying some rust coding outside of a video tutorial I recently watched.

Will use TCP sockets to create a server that will listen on TCP port 3470

The client will be able to connect to TCP port 3470 and have the keystrokes sent to 
the TCP socket instead of stdout and print an echo back of what you type.

## Server

Server compiles without errors or warnings.

To compile
rustc ./server/src/main.rs

To run
./server/src/main

## Client

Hasn't been written yet, you can use netcat to connect to the server
and test it.

In a seperate terminal run:
$ nc 127.0.0.1:3470

Whatever you type should be echoed back to you.
