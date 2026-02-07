# RUSTMOUTH
rustmouth is the high level API for the praat-sys library. It ensures the praat c++ code is
memory safe and will ideally provide a simple user interface for calling praat commands in the rust language. 

## Specifications
Level	Usage	CPU	RAM	Disk Space
Minimum	Development	Dual-core (Intel i5 / Apple M1)	8GB	5GB
Recommended	Development	8-core+ (Apple M-Series / Intel i7)	16GB - 32GB	10GB+ (SSD)
Production	Runtime	Any modern 64-bit CPU	100MB - 500MB*	~100MB


| Level | Usage | CPU | RAM | Disk Space | OS |
|----------|----------|----------|----------|----------|----------|
| Minimum | Compile | Dual-core (Intel i5 / Apple M1) | 8GB | 5GB | MacOS 11.0+ (Big Sur or newer)
| Recommended | Compile | 8-core+ (Apple M-Series / Intel i7) | 16GB - 32GB | 10GB+ SSD | MacOS 11.0+ (Big Sur or newer)
| | Runtime | Any modern 64-bit CPU | 100MB - 500MB* | ~ | MacOS 11.0+ (Big Sur or newer)

*_Memory usage depends on the data being processed_

## Supported CPU Architecture:
* Apple Silicon: ARM64 (i.e. M1, M2, M3, M4, M5)
* Intel: x86_64 (i.e. Core i3/i5/i7/i9, Xeon)
* AMD: x86_64 (i.e. Ryzen 3/5/7/9, Threadripper)

## Supported OS
* MacOS 11.0+ (Big Sur or newer)
* Linux - Work has begun but it has not been tested.
* Windows 10/11 - Work has begun but it has not been tested.

## Compile Times
On my personal device (M4 (4 performance and 6 efficiency), 24GB Unified Memory), compile times average anywhere from 1:30 - 2:00 minutes (inlcuding praat-sys).

## Stability
Though praat-sys has gone under a large "rebuild", and the link between the libpraat.a library and rust is a lot more stable, it should still be taken into account that this is experimental work. 
