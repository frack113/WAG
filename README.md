# Windows_Artefact_Generator
Generating Windows malware Artefacts for detection testing

Thanks to https://github.com/trickster0/OffensiveRust for the help.

Wag is not a TTP simulator like Redcanary, it is a simple artefact generator.
but why ?

- test your sysmon configuration
- test your EDR

# Artefact
- name pipe
- load a vunerable driver ?
- registry key ?
 
# Structure
The artefact information are stored in a json file
 ```json
 {
    "name": "Name of the malware family/test",
    "namepipe": [
        "regex 1",
        "regex x"
    ]
  }
  ```

  As we have regex in json need 2 escape level for `\`

 # Commandline

 The current commandline is for the POC can and will change.

```
Windows Artefact Generator

Usage: wag.exe [OPTIONS]

Options:
  -a, --artefact <ARTEFACT>  Name of type of artefact to genrerate [default: namepipe]
  -n, --name <NAME>          Name of the malware artefact
  -l, --list                 List all the malware Name
  -p, --place <PLACE>        Number of the place in the list
  -h, --help                 Print help
  -V, --version              Print version
```

# Exist Code
Can be usefull 

- 0 All is good
- 1 The name of the malware did not exist


# TODO LIST

- [ ] cli list of the artefact only namepipe now :)
- [ ] cli list of the tests for a malware name
- [X] Regex to string
- [ ] Doc and help
- [ ] Data class ?
- [ ] Bug
- [ ] make a better code
