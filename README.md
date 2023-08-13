# Windows_Artefact_Generator
Generating Windows malware Artefacts for detection testing

Thanks to https://github.com/trickster0/OffensiveRust 
 
 # Structure

 wag
 |-> data
 |---> test_x.json

 for the moment only "namepipe" in the json

 # Commandline

```
Windows Artefact Generator

Usage: wag.exe [OPTIONS]

Options:
  -n, --name <NAME>  Name of the malware artefact
  -l, --list         List all the malware Name
  -h, --help         Print help
  -V, --version      Print version
```

# TODO LIST

- [ ] cli list of the tests
- [X] Regex to string
- [ ] Doc and help
- [ ] Data class ?
- [ ] Artefact::load do not check if it is a json or a file
