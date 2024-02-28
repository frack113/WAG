```
__          ___           _                   
\ \        / (_)         | |                  
 \ \  /\  / / _ _ __   __| | _____      _____ 
  \ \/  \/ / | | '_ \ / _` |/ _ \ \ /\ / / __|
   \  /\  /  | | | | | (_| | (_) \ V  V /\__ \
    \/  \/   |_|_| |_|\__,_|\___/ \_/\_/ |___/
                                              
                                              
                   _        __           _   
        /\        | |      / _|         | |  
       /  \   _ __| |_ ___| |_ __ _  ___| |_ 
      / /\ \ | '__| __/ _ \  _/ _` |/ __| __|
     / ____ \| |  | ||  __/ || (_| | (__| |_ 
    /_/    \_\_|   \__\___|_| \__,_|\___|\__|
                                         
                                         
          _____                           _             
         / ____|                         | |            
        | |  __  ___ _ __   ___ _ __ __ _| |_ ___  _ __ 
        | | |_ |/ _ \ '_ \ / _ \ '__/ _` | __/ _ \| '__|
        | |__| |  __/ | | |  __/ | | (_| | || (_) | |   
         \_____|\___|_| |_|\___|_|  \__,_|\__\___/|_|   
```

# Purpose
Generating Windows malware Artefacts for detection testing

Wag is not a TTP simulator like Redcanary, it is a simple artefact generator.
but why ?

- test your sysmon configuration
- test your EDR

It is not designed to generate IOC like ip, hash ...

# Artefact

See [Artefacts file](Artefacts.md)


# How Contribute

- repport bug
- fix some code
- add new artefact
- add more example

# General Use

```cmd
wag.exe <COMMAND>
```

Example can be found here [cli_help](cli_help.md)