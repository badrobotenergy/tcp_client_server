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

This starts a tcp server listener: 127.0.0.1:3470

## Client

Client compiles without errors or warnings.

To compile run
.client/src/main.rs

In a seperate terminal run:
$  .client/src/main.rs

This connects to the server: 127.0.0.1:3470

Whatever you type should be echoed back to you.

## Testing

You can verify correct function by running tshark in a third terminal.

$ sudo tshark --color -i 3 -Y "ip.addr==127.0.0.1 and tcp.port==3470"
Running as user "root" and group "root". This could be dangerous.
Capturing on 'Loopback: lo'
 ** (tshark:61508) 20:24:20.008554 [Main MESSAGE] -- Capture started.
 ** (tshark:61508) 20:24:20.008631 [Main MESSAGE] -- File: "/tmp/wireshark_loNS8UK2.pcapng"
   72 219.872872115    127.0.0.1 → 127.0.0.1    TCP 74 55472 → 3470 [SYN] Seq=0 Win=33280 Len=0 MSS=65495 SACK_PERM=1 TSval=4064640773 TSecr=0 WS=128
   73 219.872881355    127.0.0.1 → 127.0.0.1    TCP 54 3470 → 55472 [RST, ACK] Seq=1 Ack=1 Win=0 Len=0


