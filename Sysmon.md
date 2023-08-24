# Sysmon V15 Artefact

❎ Wag will not create artefact
✅ Wag can create artefact
❓ Need to be check

| EventID | Description | Cover by wag
| --- | --- | --- |
| 1 | Process creation | ❎
| 2 | process changed a file creation time | ❓
| 3 | Network connection | ❎
| 4 | Sysmon service state changed | ❎
| 5 | Process terminated | ❎
| 6 | Driver loaded | ✅
| 7 | Image loaded | ❓
| 8 | CreateRemoteThread | ❓
| 9 | RawAccessRead | ❓
| 10 | ProcessAccess
| 11 | FileCreate | ✅
| 12 | RegistryEvent (Object create and delete) | ❓
| 13 | RegistryEvent (Value Set) | ❓
| 14 | RegistryEvent (Key and Value Rename) | ❓
| 15 | FileCreateStreamHash | ❓
| 16 | ServiceConfigurationChange | ❎
| 17 | PipeEvent (Pipe Created) | ✅
| 18 | PipeEvent (Pipe Connected) | ❓
| 19 | WmiEvent (WmiEventFilter activity detected) | ❓
| 20 | WmiEvent (WmiEventConsumer activity detected) | ❓
| 21 | WmiEvent (WmiEventConsumerToFilter activity detected) | ❓
| 22 | DNSEvent (DNS query) | ❎
| 23 | FileDelete (File Delete archived) | ❓
| 24 | ClipboardChange (New content in the clipboard) | ❓
| 25 | ProcessTampering (Process image change) | ❓
| 26 | FileDeleteDetected (File Delete logged) | ❓
| 27 | FileBlockExecutable | ❓
| 28 | FileBlockShredding | ❓
| 29 | FileExecutableDetected | ❓
| 255 | Error | ❎

## Process creation (1)
Cover by other tools like Atomic RedTeam

## process changed a file creation time (2)
Need to see its usefulness

## Network connection (3)
Cover by other tools like Atomic RedTeam

## Sysmon service state changed (4)
Need to see its usefulness

## Process terminated (5)
Cover by other tools like Atomic RedTeam

## Driver loaded (6)
Done by the option X

## Image loaded (7)
Need to see its usefulness

## CreateRemoteThread (8)
Need to see its usefulness

## RawAccessRead (9)
Need to see its usefulness

## ProcessAccess (10)
Need to see its usefulness

## FileCreate (11)
Done by the option X

## RegistryEvent (12,13,14)
Cover by other tools like Atomic RedTeam

## FileCreateStreamHash (15)
Need to find the Bug in rust and sysmon 

## ServiceConfigurationChange (16)
Need to see its usefulness

## PipeEvent (17,18)
Only Pipe Created , no Pipe Connected

## WmiEvent (19,20,21)
Need to see its usefulness

## DNSEvent (22)
Cover by other tools like Atomic RedTeam

## FileDelete (23)
Need to see its usefulness

## ClipboardChange (24)
Need to see its usefulness

## ProcessTampering (25)
Need to see its usefulness

## FileDeleteDetected (26)
Need to see its usefulness

## FileBlockExecutable (27)
Need to see its usefulness

## FileBlockShredding (28)
Need to see its usefulness

## FileExecutableDetected (29)
Need to see its usefulness

## Error (255)
Need to see its usefulness
