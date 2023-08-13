# Windows_Artefact_Generator
Generating Windows malware Artefacts for detection testing

Thanks to https://github.com/trickster0/OffensiveRust 
 
 # Structure

 wag
 |-> data
 |---> test_x.json

 for the moment only "namepipe" in the json

 # Commandline

wag name_of_the_test

next ->  -n lauch the namepipe generation

# TODO LIST

- [ ] cli list of the tests
- [ ] Regex to string
- [ ] Doc and help
- [ ] Data class ?
- [ ] Artefact::load do not check if it is a json or a file
