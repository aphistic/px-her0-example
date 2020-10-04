MEMORY
{
    # Got these values from the STM32L0x2 reference manual's memory map
    # as well as the Flash and RAM details of the PX-HER0's STM32L072RB:
    # https://www.st.com/resource/en/reference_manual/dm00108281-ultralowpower-stm32l0x2-advanced-armbased-32bit-mcus-stmicroelectronics.pdf
    
    FLASH : ORIGIN = 0x08000000, LENGTH = 128K
    RAM : ORIGIN = 0x20000000, LENGTH = 20K
}