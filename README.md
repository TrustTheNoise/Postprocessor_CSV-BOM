This is a program for compiling one common specification for several projects. Two or more files of the format are 
submitted to the input .csv and at the output we get a file format .csv with the general specification. 
In this case, all postfixes will be deleted and (if desired) displayed when deleted.

Flags:
-f if the flag is not specified - if different data is specified in the same columns for the same elements, these elements are added to the output table in separate rows
if the flag is specified, even if different data is specified in the same columns for the same elements, the elements must be combined into one row, and the field where the conflict occurs should be replaced with the “Forced merge” line
 
-wo if the flag is specified, then deleted postfixes will not be output to the terminal

-h outputs this message to the terminal