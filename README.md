# YARS
YARS: Yet Another Ransomware Simulator

## Usage
### Using Cargo:
```
cargo run -- <MODE> <INPUT_FILE>
```
### From exe:
```
.\YARS.exe <MODE> <INPUT FILE>
```
Example:
```
.\YARS.exe prepare file_paths.txt

.\YARS.exe encrypt file_paths.txt

.\YARS.exe decrypt file_paths.txt

.\YARS.exe cleanup .\test.txt
```
