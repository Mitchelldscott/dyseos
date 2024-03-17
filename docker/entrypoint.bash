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

DATETIME=$(date '+%d%h%y%H%M')
PROJECT_ROOT=${PWD}

if [[ ! -f ${PROJECT_ROOT}/Cargo.toml ]]; then
    echo -e "${BRed}No Cargo.toml, Exit${NC}"
    exit 0
fi

DUMP="false"
CLEAN="false"
QEMU="false"
BUILD="true"
FEATURES=""
DEBUGGER="false"

function usage {

    echo -e "\t${BCyan}Devbox Usage${NC}\n"
    echo -e "${BGreen}Usage${NC}\tentrypoint.bash <OPTIONS> <COMMAND>\n"
    echo -e "${BGreen}Compose${NC}\tdocker compose run <SERVICE> <OPTIONS> <COMMAND>\n"
    echo -e "${BGreen}Options:${NC}"
    echo -e "  ${BBlue}-c${NC} Clean the workspace"
    echo -e "  ${BBlue}-d${NC} Dump kernel elf info"
    echo -e "  ${BBlue}-b${NC} Cargo binary to build, from Cargo.toml"
    echo -e "  ${BBlue}-n${NC} Don't Build the elf or run cargo objcopy"
    echo -e "  ${BBlue}-q${NC} Run QEMU emulator, requires that bin is the kernel image"
    echo -e "  ${BBlue}-g${NC} Run QEMU emulator and attach LLDB session, overrides -q"
    echo -e "  ${BBlue}-p <PROFILE>${NC} Cargo profile to build"
    echo -e "  ${BBlue}-f <FEATURES>${NC} Cargo features to enable"
    echo -e "${BGreen}Command${NC}\teverything after the last option is interpreted as a bash command after the tools execute\n"

}

while getopts "bndcqgp:f:h" flag; do
    case $flag in
        b) # Cargo binary to build
            BIN=${OPTARG}
        ;;
        n) # Don't Build the elf or run cargo objcopy
            BUILD="false"
        ;;
        d) # Dump kernel elf info
            DUMP="true"
        ;;
        c) # Clean the workspace
            CLEAN="true"
        ;;
        q) # Run qemu emulator
            QEMU="true"
        ;;
        g) # run qemu emulator with debugger (forces -q)
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
        h)
            usage
            exit 0
        ;;
        \?) # Handle invalid options
            usage
        ;;
    esac
done

shift $(( OPTIND - 1 ))

if [[ ${BIN} = "" ]]; then
    BIN_ARG=""
    DATA_DIR=${PROJECT_ROOT}/dump
else
    BIN_ARG="--bin ${BIN} "
    DATA_DIR=${PROJECT_ROOT}/${BIN}_dump

    if [[ ! -d ${DATA_DIR} ]]; then
        mkdir ${DATA_DIR}
    fi
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
echo -e "  ${BGreen}CMD${NC}:\t\t$1\n"

if [[ ${CLEAN} = true ]]; then

    echo -e "\n\t${BCyan}Cleaning Project${NC}"
    cargo clean ${PROFILE_ARG}${TARGET_ARG}

    if [[ -f ${DATA_DIR}/* ]]; then
        rm ${DATA_DIR}/*
    fi

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
    echo "===== objdump =====" > ${DATA_DIR}/elf_info_${DATETIME}.txt
    cargo objdump ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}-- -d >> ${DATA_DIR}/elf_info_${DATETIME}.txt
    echo "===== nm =====" >> ${DATA_DIR}/elf_info_${DATETIME}.txt
    cargo nm ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}>> ${DATA_DIR}/elf_info_${DATETIME}.txt
    echo "===== size =====" >> ${DATA_DIR}/elf_info_${DATETIME}.txt
    cargo size ${BIN_ARG}${PROFILE_ARG}${TARGET_ARG}>> ${DATA_DIR}/elf_info_${DATETIME}.txt
    cargo modules structure ${BIN_ARG} > ${DATA_DIR}/structure_${DATETIME}.txt
    cargo modules dependencies ${BIN_ARG} --no-sysroot > ${DATA_DIR}/depends.dot
    dot -Tpng ${DATA_DIR}/depends.dot -o ${DATA_DIR}/depends_${DATETIME}.png
    
    if [[ -f "${PROJECT_ROOT}/src/lib.rs" ]]; then
    
        cargo modules structure --lib >> ${DATA_DIR}/structure_${DATETIME}.txt
        cargo modules dependencies --lib --no-sysroot > ${DATA_DIR}/lib_depends.dot
        dot -Tpng ${DATA_DIR}/lib_depends.dot -o ${DATA_DIR}/lib_depends_${DATETIME}.png

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

if [[ $1 != "" && $1 != " " ]]; then

    echo -e "\n\t${BCyan}System Command${NC}\n$@\n"

    $@

fi