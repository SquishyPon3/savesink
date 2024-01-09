# Savesink
Yes the name is intentional...

## Description 
Savesink is a CLI program for managing save data between devices. 
Intended to provide save transferring, backups, and simple version control. 

## Setup
WIP - CURRENLTY NON-FUNCTIONING !!
Savesink can either be set up as a p2p or client server architecture depending on needs.  
(Probably only Client / Server)  

1. Client 1 & 2 / Server: `cargo install savesink --path "path/to/savesink/"` to install Savesink on each client machine and a server machine.
2. Client 1 & 2 / Server: `savesink create` to create a local save tracker folder on each machine.
3. Client 1: `savesink add -name "game_name" -source "game/save/source/directory"` to create a tracked file for a particular game save path.
4. Client 1: `savesink commit` to commit local save changes from the source directory to a new folder in the local save tracker.
5. Client 1: `savesink push` to push local save tracker data changes to the remote server's save tracker.
6. Client 2: `savesink pull` to pull remote server's save tracker data, `--force` to override local committed changes.
7. Client 2: `savesink sync` to replace local source directory save data with tracked save data.
