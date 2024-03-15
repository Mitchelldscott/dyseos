#! /bin/bash

# Reset
NC='\033[0m'       # Text Reset

# Regular Colors
Black='\033[0;30m'        # Black
Red='\033[0;31m'          # Red
Green='\033[0;32m'        # Green
Yellow='\033[0;33m'       # Yellow
Blue='\033[0;34m'         # Blue
Purple='\033[0;35m'       # Purple
Cyan='\033[0;36m'         # Cyan
White='\033[0;37m'        # White

# Bold
BBlack='\033[1;30m'       # Black
BRed='\033[1;31m'         # Red
BGreen='\033[1;32m'       # Green
BYellow='\033[1;33m'      # Yellow
BBlue='\033[1;34m'        # Blue
BPurple='\033[1;35m'      # Purple
BCyan='\033[1;36m'        # Cyan
BWhite='\033[1;37m'       # White

DUMP="false"
CLEAN="false"
QEMU="false"
BUILD="true"
FEATURES=""
DEBUGGER="false"

while getopts "bndcqgp:f:" flag; do
    case $flag in
        b) # Cargo binary to build
            BIN=${OPTARG}
        ;;
        n) # build the elf and copy to img, defaults to true; the opposite of dump and clean
            BUILD="false"
        ;;
        d) # dump kernel elf info
            DUMP="true"
        ;;
        c) # clean build
            CLEAN="true"
        ;;
        q) # run qemu emulator
            QEMU="true"
        ;;
        q) # run qemu emulator with debugger (forces -q)
            QEMU="true"
            DEBUGGER="true"
        ;;
        p) # override cargo profile from compose.yaml
            PROFILE=${OPTARG}
        ;;
        f) # cargo features
            FEATURES=${OPTARG}
            FEATURES_ARG=--features=${OPTARG}
        ;;
        \?) # Handle invalid options
        ;;
    esac
done

shift $(( OPTIND - 1 ))

if [[ ${BIN} = "" ]]; then
    BIN_ARG=""
else
    BIN_ARG="--bin ${BIN} "
fi

if [[ ${TARGET} = none ]]; then
    TARGET_ARG=""
else
    TARGET_ARG="--target=${TARGET} "
fi

if [[ ${PROFILE} = debug ]]; then
    PROFILE_ARG=""
else
    PROFILE_ARG="--profile=${PROFILE} "
fi

echo -e "\t${BCyan}DyseOS DevBox ${BIN}"
echo -e "  ${BGreen}Architecture${NC}:\t${TARGET}"
echo -e "  ${BGreen}Profile${NC}:\t${PROFILE}"
echo -e "  ${BGreen}Features${NC}:\t${FEATURES}"
echo -e "  ${BGreen}Machine${NC}:\t${MACHINE}"
echo -e "  ${BGreen}CMD${NC}:\t\t$1"

if [[ ${CLEAN} = true ]]; then

    echo -e "\n\t${BCyan}Cleaning Project${NC}"
    cargo clean ${PROFILE_ARG}${TARGET_ARG}
    rm /home/dev/dyseos/${BIN}*

fi

if [[ $BUILD = true ]]; then

    echo -e "\n\t${BCyan}Building Project${NC}"
    cargo build ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}${FEATURES_ARG}

    if [[ $? != 0 ]]; then

        bash
        exit 0

    fi

    cargo objcopy ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}-- -O binary ${BIN}.img

fi

if [[ ${DUMP} = true && ${BIN} != "" ]]; then 
    
    echo -e "\n\t${BCyan}Dumping ELF info${NC}"
    echo "===== objdump =====" > /home/dev/dyseos/${BIN}_elf_info.txt
    cargo objdump ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}-- -s -d >> /home/dev/dyseos/${BIN}_elf_info.txt
    echo "===== nm =====" >> /home/dev/dyseos/${BIN}_elf_info.txt
    cargo nm ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}>> /home/dev/dyseos/${BIN}_elf_info.txt
    echo "===== size =====" >> /home/dev/dyseos/${BIN}_elf_info.txt
    cargo size ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}>> /home/dev/dyseos/${BIN}_elf_info.txt
    cargo modules structure ${BIN_ARG} > ${BIN}_structure.txt
    
    if [[ -f "${PROJECT_ROOT}/src/lib.rs" ]]; then
    
        cargo modules structure --lib >> ${BIN}_structure.txt

    fi

fi

if [[ ${MACHINE} != none && ${QEMU} = true ]]; then
    
    echo -e "\n\t${BCyan}Launching Qemu${NC}"
    
    if [[ ${DEBUGGER} = true ]]; then

        qemu-system-aarch64 -M ${MACHINE} -kernel ${PROJECT_ROOT}/${BIN}.img -gdb tcp::1234 -S -serial stdio -display none &
        lldb --one-line "gdb-remote localhost:1234" target/${TARGET}/${PROFILE}/${BIN}

    else

        qemu-system-aarch64 -M ${MACHINE} -serial stdio -display none -kernel ${PROJECT_ROOT}/${BIN}.img

    fi

fi

if [[ $1 != "" ]]; then

    $@

fi