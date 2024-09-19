PACKAGES=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata bios
PACKAGE_DIRS=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libc /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libcompiler_rt /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libbase /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libfatfs /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitespi /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitedram /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libliteeth /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesdcard /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesata /run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/bios
LIBS=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata
TRIPLE=riscv64-unknown-elf
CPU=vexriscv
CPUFAMILY=riscv
CPUFLAGS= -march=rv32i2p0_mafdc -mabi=ilp32d -D__vexriscv__ -DUART_POLLING
CPUENDIANNESS=little
CLANG=0
CPU_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/cores/cpu/vexriscv_smp
SOC_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc
PICOLIBC_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/pythondata-software-picolibc/pythondata_software_picolibc/data
PICOLIBC_FORMAT=integer
COMPILER_RT_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/pythondata-software-compiler_rt/pythondata_software_compiler_rt/data
export BUILDINC_DIRECTORY
BUILDINC_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/build/litex/software/include
LIBC_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libc
LIBCOMPILER_RT_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libcompiler_rt
LIBBASE_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libbase
LIBFATFS_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libfatfs
LIBLITESPI_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitespi
LIBLITEDRAM_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitedram
LIBLITEETH_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/libliteeth
LIBLITESDCARD_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesdcard
LIBLITESATA_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesata
BIOS_DIRECTORY=/run/media/bitten2up/04da6281-72b6-4fec-89e9-976a408c0094/bitten2up/riscv/openfpga-litex/litex/vendor/litex/litex/soc/software/bios
LTO=0
BIOS_CONSOLE_LITE=1