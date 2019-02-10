EESchema Schematic File Version 4
LIBS:monotron-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 3
Title "Monotron 32K Schematic"
Date "2019-02-10"
Rev "0.8.0"
Comp "https://github.com/thejpster/monotron"
Comment1 "Copyright (c) Jonathan 'theJPster' Pallant 2019"
Comment2 "CC BY-SA 4.0"
Comment3 ""
Comment4 ""
$EndDescr
Text HLabel 4150 950  0    50   Output ~ 0
IO_TO_MCU
$Comp
L Connector:Mini-DIN-6 J14
U 1 1 5C86652F
P 9000 2750
F 0 "J14" H 9000 3117 50  0000 C CNN
F 1 "Mini-DIN-6" H 9000 3026 50  0000 C CNN
F 2 "Monotron:mini-DIN 6" H 9000 2750 50  0001 C CNN
F 3 "https://www.te.com/commerce/DocumentDelivery/DDEController?Action=srchrtrv&DocNm=5749180&DocType=Customer+Drawing&DocLang=English" H 9000 2750 50  0001 C CNN
F 4 "5749180-1-ND" H 9000 2750 50  0001 C CNN "Digikey"
	1    9000 2750
	1    0    0    -1  
$EndComp
$Comp
L Connector:Mini-DIN-6 J15
U 1 1 5C866618
P 9000 3900
F 0 "J15" H 9000 4267 50  0000 C CNN
F 1 "Mini-DIN-6" H 9000 4176 50  0000 C CNN
F 2 "Monotron:mini-DIN 6" H 9000 3900 50  0001 C CNN
F 3 "https://www.te.com/commerce/DocumentDelivery/DDEController?Action=srchrtrv&DocNm=5749180&DocType=Customer+Drawing&DocLang=English" H 9000 3900 50  0001 C CNN
F 4 "5749180-1-ND" H 9000 3900 50  0001 C CNN "Digikey"
	1    9000 3900
	1    0    0    -1  
$EndComp
$Comp
L Connector:DB25_Female J13
U 1 1 5C866771
P 7200 3150
F 0 "J13" H 7400 3200 50  0000 L CNN
F 1 "DB25_Female" H 7400 3100 50  0000 L CNN
F 2 "Connector_Dsub:DSUB-25_Male_Horizontal_P2.77x2.84mm_EdgePinOffset7.70mm_Housed_MountingHolesOffset9.12mm" H 7200 3150 50  0001 C CNN
F 3 "http://portal.fciconnect.com/Comergent//fci/drawing/c-dd-0024.pdf" H 7200 3150 50  0001 C CNN
F 4 "609-5920-ND" H 7200 3150 50  0001 C CNN "Digikey"
	1    7200 3150
	1    0    0    -1  
$EndComp
Text Notes 8850 2300 0    50   ~ 0
Keyboard
Text Notes 8900 3450 0    50   ~ 0
Mouse
Wire Wire Line
	6900 1950 6350 1950
Wire Wire Line
	6900 4250 6750 4250
Wire Wire Line
	6750 4250 6750 4050
Wire Wire Line
	6750 4050 6900 4050
Wire Wire Line
	6750 4050 6750 3850
Wire Wire Line
	6750 3850 6900 3850
Connection ~ 6750 4050
Wire Wire Line
	6750 3850 6750 3650
Wire Wire Line
	6750 3650 6900 3650
Connection ~ 6750 3850
Wire Wire Line
	6750 3650 6750 3450
Wire Wire Line
	6750 3450 6900 3450
Connection ~ 6750 3650
Wire Wire Line
	6900 3250 6750 3250
Wire Wire Line
	6750 3250 6750 3450
Connection ~ 6750 3450
Wire Wire Line
	6900 3050 6750 3050
Wire Wire Line
	6750 3050 6750 3250
Connection ~ 6750 3250
Wire Wire Line
	6900 2850 6750 2850
Wire Wire Line
	6750 2850 6750 3050
Connection ~ 6750 3050
$Comp
L power:GND #PWR035
U 1 1 5C870121
P 6750 4550
F 0 "#PWR035" H 6750 4300 50  0001 C CNN
F 1 "GND" H 6755 4377 50  0000 C CNN
F 2 "" H 6750 4550 50  0001 C CNN
F 3 "" H 6750 4550 50  0001 C CNN
	1    6750 4550
	1    0    0    -1  
$EndComp
Wire Wire Line
	6750 4250 6750 4550
Connection ~ 6750 4250
Wire Wire Line
	6900 2050 6350 2050
Wire Wire Line
	6900 2150 6350 2150
Wire Wire Line
	6900 2250 6350 2250
Wire Wire Line
	6900 2350 6350 2350
Wire Wire Line
	6900 2450 6350 2450
Wire Wire Line
	6900 2550 6350 2550
Wire Wire Line
	6900 2750 6350 2750
Wire Wire Line
	6900 2950 6350 2950
Wire Wire Line
	6900 3150 6350 3150
Wire Wire Line
	6900 3350 6350 3350
Wire Wire Line
	6900 3550 6350 3550
Wire Wire Line
	6900 3750 6350 3750
Wire Wire Line
	6900 3950 6350 3950
Wire Wire Line
	6900 4150 6350 4150
Wire Wire Line
	6900 4350 6350 4350
Text Label 6400 1950 0    50   ~ 0
STROBE
Text Label 6400 2150 0    50   ~ 0
D0
Text Label 6400 2350 0    50   ~ 0
D1
Text Label 6400 2550 0    50   ~ 0
D2
Text Label 6400 2750 0    50   ~ 0
D3_MOSI
Text Label 6400 2950 0    50   ~ 0
D4_MISO
Text Label 6400 3150 0    50   ~ 0
D5_SCK
Text Label 6400 3350 0    50   ~ 0
D6
Text Label 6400 3550 0    50   ~ 0
D7
Text Label 6400 3950 0    50   ~ 0
BUSY
Text Label 6400 4150 0    50   ~ 0
PE
Text Label 6400 4350 0    50   ~ 0
SEL
Text Label 6400 2050 0    50   ~ 0
AUTOF
Text Label 6400 2250 0    50   ~ 0
ERROR
Text Label 6400 2450 0    50   ~ 0
INIT
Text Label 6400 2650 0    50   ~ 0
SELPRIN
NoConn ~ 8700 2650
NoConn ~ 8700 3800
NoConn ~ 8700 2850
NoConn ~ 8700 4000
Wire Wire Line
	9300 2650 9650 2650
Wire Wire Line
	9300 2850 9650 2850
Wire Wire Line
	9300 3800 9650 3800
Wire Wire Line
	9300 4000 9650 4000
$Comp
L power:GND #PWR041
U 1 1 5C89AC5C
P 9400 4100
F 0 "#PWR041" H 9400 3850 50  0001 C CNN
F 1 "GND" H 9405 3927 50  0000 C CNN
F 2 "" H 9400 4100 50  0001 C CNN
F 3 "" H 9400 4100 50  0001 C CNN
	1    9400 4100
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR040
U 1 1 5C89AC8A
P 9400 3000
F 0 "#PWR040" H 9400 2750 50  0001 C CNN
F 1 "GND" H 9405 2827 50  0000 C CNN
F 2 "" H 9400 3000 50  0001 C CNN
F 3 "" H 9400 3000 50  0001 C CNN
	1    9400 3000
	1    0    0    -1  
$EndComp
Wire Wire Line
	9300 2750 9400 2750
Wire Wire Line
	9400 2750 9400 3000
Wire Wire Line
	9300 3900 9400 3900
Wire Wire Line
	9400 3900 9400 4100
$Comp
L power:+5V #PWR036
U 1 1 5C89CCFE
P 8450 2600
F 0 "#PWR036" H 8450 2450 50  0001 C CNN
F 1 "+5V" H 8465 2773 50  0000 C CNN
F 2 "" H 8450 2600 50  0001 C CNN
F 3 "" H 8450 2600 50  0001 C CNN
	1    8450 2600
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR037
U 1 1 5C89CD2C
P 8450 3750
F 0 "#PWR037" H 8450 3600 50  0001 C CNN
F 1 "+5V" H 8465 3923 50  0000 C CNN
F 2 "" H 8450 3750 50  0001 C CNN
F 3 "" H 8450 3750 50  0001 C CNN
	1    8450 3750
	1    0    0    -1  
$EndComp
Wire Wire Line
	8700 3900 8450 3900
Wire Wire Line
	8450 3900 8450 3750
Wire Wire Line
	8700 2750 8450 2750
Wire Wire Line
	8450 2750 8450 2600
Text Label 9450 2650 0    50   ~ 0
KB_CLK
Text Label 9450 2850 0    50   ~ 0
KB_DATA
Text Label 9500 3800 0    50   ~ 0
MS_CLK
Text Label 9500 4000 0    50   ~ 0
MS_DATA
Text Label 6400 3750 0    50   ~ 0
ACK
Wire Wire Line
	4550 2850 5150 2850
Wire Wire Line
	4550 2950 5150 2950
Text Label 4650 2850 0    50   ~ 0
KB_CLK
Text Label 4650 2950 0    50   ~ 0
MS_CLK
Wire Wire Line
	4550 3050 5150 3050
Text Label 4650 3050 0    50   ~ 0
KB_DATA
Text Label 4650 3150 0    50   ~ 0
MS_DATA
Text HLabel 4150 1050 0    50   Input ~ 0
MCU_TO_IO
$Comp
L power:+5V #PWR033
U 1 1 5C61C941
P 3950 1500
F 0 "#PWR033" H 3950 1350 50  0001 C CNN
F 1 "+5V" H 3965 1673 50  0000 C CNN
F 2 "" H 3950 1500 50  0001 C CNN
F 3 "" H 3950 1500 50  0001 C CNN
	1    3950 1500
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR034
U 1 1 5C61CB48
P 3950 4650
F 0 "#PWR034" H 3950 4400 50  0001 C CNN
F 1 "GND" H 3955 4477 50  0000 C CNN
F 2 "" H 3950 4650 50  0001 C CNN
F 3 "" H 3950 4650 50  0001 C CNN
	1    3950 4650
	1    0    0    -1  
$EndComp
Text Label 4250 1050 0    50   ~ 0
UART_RX
Text Label 4250 950  0    50   ~ 0
UART_TX
$Comp
L power:+5V #PWR038
U 1 1 5C7704F7
P 9150 5200
F 0 "#PWR038" H 9150 5050 50  0001 C CNN
F 1 "+5V" H 9165 5373 50  0000 C CNN
F 2 "" H 9150 5200 50  0001 C CNN
F 3 "" H 9150 5200 50  0001 C CNN
	1    9150 5200
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR039
U 1 1 5C770515
P 9150 5700
F 0 "#PWR039" H 9150 5450 50  0001 C CNN
F 1 "GND" H 9155 5527 50  0000 C CNN
F 2 "" H 9150 5700 50  0001 C CNN
F 3 "" H 9150 5700 50  0001 C CNN
	1    9150 5700
	1    0    0    -1  
$EndComp
$Comp
L Device:C C12
U 1 1 5C77054A
P 8950 5450
F 0 "C12" H 9000 5550 50  0000 L CNN
F 1 "100n" H 9000 5350 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 8988 5300 50  0001 C CNN
F 3 "~" H 8950 5450 50  0001 C CNN
	1    8950 5450
	1    0    0    -1  
$EndComp
$Comp
L Device:C C13
U 1 1 5C77061D
P 9350 5450
F 0 "C13" H 9400 5550 50  0000 L CNN
F 1 "100n" H 9400 5350 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 9388 5300 50  0001 C CNN
F 3 "~" H 9350 5450 50  0001 C CNN
	1    9350 5450
	1    0    0    -1  
$EndComp
Wire Wire Line
	8950 5600 8950 5650
Wire Wire Line
	8950 5650 9150 5650
Wire Wire Line
	9150 5650 9150 5700
Connection ~ 9150 5650
Wire Wire Line
	8950 5300 8950 5250
Wire Wire Line
	8950 5250 9150 5250
Wire Wire Line
	9150 5250 9150 5200
Connection ~ 9150 5250
Wire Wire Line
	9350 5650 9350 5600
Wire Wire Line
	9150 5650 9350 5650
Wire Wire Line
	9350 5300 9350 5250
Wire Wire Line
	9150 5250 9350 5250
$Comp
L MCU_Microchip_ATmega:ATmega48A-PU U5
U 1 1 5C31A32D
P 3950 3150
F 0 "U5" H 3310 3196 50  0000 R CNN
F 1 "ATmega48A-PU" H 3310 3105 50  0000 R CNN
F 2 "Package_DIP:DIP-28_W7.62mm" H 3950 3150 50  0001 C CIN
F 3 "http://ww1.microchip.com/downloads/en/DeviceDoc/ATmega48A_88A_168A-Data-Sheet-40002007A.pdf" H 3950 3150 50  0001 C CNN
	1    3950 3150
	1    0    0    -1  
$EndComp
Wire Wire Line
	3950 1650 3950 1550
Wire Wire Line
	4050 1650 4050 1550
Wire Wire Line
	4050 1550 3950 1550
Connection ~ 3950 1550
Wire Wire Line
	3950 1550 3950 1500
Wire Wire Line
	4550 3650 5150 3650
Wire Wire Line
	4550 3750 5150 3750
Text Label 4650 3650 0    50   ~ 0
UART_RX
Text Label 4650 3750 0    50   ~ 0
UART_TX
Wire Wire Line
	4550 3350 5150 3350
Wire Wire Line
	4550 3250 5150 3250
Wire Wire Line
	4550 2250 5150 2250
Wire Wire Line
	4550 2150 5150 2150
Wire Wire Line
	4550 2050 5150 2050
Wire Wire Line
	4550 1950 5150 1950
Wire Wire Line
	4550 3850 5150 3850
Wire Wire Line
	4550 3950 5150 3950
Wire Wire Line
	4550 4050 5150 4050
Wire Wire Line
	4550 4150 5150 4150
Wire Wire Line
	4550 4250 5150 4250
Wire Wire Line
	4550 4350 5150 4350
Wire Wire Line
	4550 2350 5150 2350
Wire Wire Line
	4550 2450 5150 2450
Wire Wire Line
	4550 2550 5150 2550
Wire Wire Line
	4550 2650 5150 2650
Wire Wire Line
	4550 3150 5150 3150
Text Label 4650 1950 0    50   ~ 0
D0
Text Label 4650 2050 0    50   ~ 0
D1
Text Label 4650 2150 0    50   ~ 0
D2
Text Label 4650 2250 0    50   ~ 0
D3_MOSI
Text Label 4650 2350 0    50   ~ 0
D4_MISO
Text Label 4650 2450 0    50   ~ 0
D5_SCK
Text Label 4650 2550 0    50   ~ 0
D6
Text Label 4650 2650 0    50   ~ 0
D7
Text Label 4650 3850 0    50   ~ 0
ACK
Text Label 4650 3950 0    50   ~ 0
BUSY
Text Label 4650 4050 0    50   ~ 0
PE
Text Label 4650 4150 0    50   ~ 0
ERROR
Text Label 4650 4250 0    50   ~ 0
AUTOF
Text Label 4650 4350 0    50   ~ 0
STROBE
Text Label 4650 3250 0    50   ~ 0
SELPRIN
Text Label 4650 3350 0    50   ~ 0
SEL
Wire Wire Line
	4550 3450 5150 3450
Text Label 4650 3450 0    50   ~ 0
ATMEL_RESET
Wire Wire Line
	3950 1550 3150 1550
Wire Wire Line
	3150 1550 3150 1950
Wire Wire Line
	3150 1950 3350 1950
$Comp
L Device:C C14
U 1 1 5C35D035
P 9700 5450
F 0 "C14" H 9750 5550 50  0000 L CNN
F 1 "100n" H 9750 5350 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 9738 5300 50  0001 C CNN
F 3 "~" H 9700 5450 50  0001 C CNN
	1    9700 5450
	1    0    0    -1  
$EndComp
Wire Wire Line
	9350 5650 9700 5650
Connection ~ 9350 5650
Wire Wire Line
	9700 5600 9700 5650
Wire Wire Line
	9350 5250 9700 5250
Wire Wire Line
	9700 5250 9700 5300
Connection ~ 9350 5250
Text Notes 7400 3350 0    50   ~ 0
25-pin Parallel Printer Port
$Comp
L Connector_Generic:Conn_02x03_Odd_Even J17
U 1 1 5CDAE253
P 5700 5700
F 0 "J17" H 5750 6017 50  0000 C CNN
F 1 "2x3 Header" H 5750 5926 50  0000 C CNN
F 2 "" H 5700 5700 50  0001 C CNN
F 3 "~" H 5700 5700 50  0001 C CNN
	1    5700 5700
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0117
U 1 1 5CDAE2B1
P 6250 5850
F 0 "#PWR0117" H 6250 5600 50  0001 C CNN
F 1 "GND" H 6255 5677 50  0000 C CNN
F 2 "" H 6250 5850 50  0001 C CNN
F 3 "" H 6250 5850 50  0001 C CNN
	1    6250 5850
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR0118
U 1 1 5CDAE2CC
P 6250 5450
F 0 "#PWR0118" H 6250 5300 50  0001 C CNN
F 1 "+5V" H 6265 5623 50  0000 C CNN
F 2 "" H 6250 5450 50  0001 C CNN
F 3 "" H 6250 5450 50  0001 C CNN
	1    6250 5450
	1    0    0    -1  
$EndComp
Wire Wire Line
	6250 5850 6250 5800
Wire Wire Line
	6250 5800 6000 5800
Wire Wire Line
	6000 5600 6250 5600
Wire Wire Line
	6250 5600 6250 5450
Wire Wire Line
	6000 5700 6550 5700
Wire Wire Line
	5500 5800 4850 5800
Text Label 4900 5800 0    50   ~ 0
ATMEL_RESET
Text Notes 5300 5300 0    50   ~ 0
Atmel ISP Header\nUnplug printer first!
Wire Wire Line
	6900 2650 6350 2650
Text HLabel 4150 1150 0    50   Input ~ 0
LPT_INIT
Text Label 4250 1150 0    50   ~ 0
INIT_3V3
$Comp
L 74xx:74LS14 U?
U 6 1 5CDE45D6
P 5200 1150
AR Path="/5C3C82B1/5CDE45D6" Ref="U?"  Part="6" 
AR Path="/5C821310/5CDE45D6" Ref="U7"  Part="6" 
F 0 "U7" H 5200 1150 50  0000 R CNN
F 1 "74LS14" H 5300 1350 50  0000 R CNN
F 2 "Package_DIP:DIP-14_W7.62mm" H 5200 1150 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS14" H 5200 1150 50  0001 C CNN
	6    5200 1150
	1    0    0    -1  
$EndComp
$Comp
L 74xx:74LS14 U?
U 5 1 5CDE45DD
P 5900 1150
AR Path="/5CDE45DD" Ref="U?"  Part="5" 
AR Path="/5C3C82B1/5CDE45DD" Ref="U?"  Part="5" 
AR Path="/5C821310/5CDE45DD" Ref="U7"  Part="5" 
F 0 "U7" H 5900 1150 50  0000 R CNN
F 1 "74LS14" H 6000 1350 50  0000 R CNN
F 2 "Package_DIP:DIP-14_W7.62mm" H 5900 1150 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS14" H 5900 1150 50  0001 C CNN
	5    5900 1150
	1    0    0    -1  
$EndComp
Wire Wire Line
	4150 950  4750 950 
Wire Wire Line
	4150 1050 4750 1050
Wire Wire Line
	4150 1150 4900 1150
Wire Wire Line
	5500 1150 5600 1150
Wire Wire Line
	6200 1150 6600 1150
Text Label 6350 1150 0    50   ~ 0
INIT
Text Notes 6250 950  0    50   ~ 0
The Atmel is one pin short, so we drive nINIT\nfrom the MCU, using these hex inverters\nas a level shifter
Wire Wire Line
	5500 5700 4850 5700
Wire Wire Line
	5500 5600 4850 5600
Text Label 4900 5600 0    50   ~ 0
D4_MISO
Text Label 4900 5700 0    50   ~ 0
D5_SCK
Text Label 6050 5700 0    50   ~ 0
D3_MOSI
$EndSCHEMATC
