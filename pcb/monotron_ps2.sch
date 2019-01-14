EESchema Schematic File Version 4
LIBS:monotron-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 3
Title "Monotron 32K Schematic"
Date "2019-01-13"
Rev "0.6.0"
Comp "https://github.com/thejpster/monotron"
Comment1 "Copyright (c) Jonathan 'theJPster' Pallant 2019"
Comment2 "CC BY-SA 4.0"
Comment3 ""
Comment4 ""
$EndDescr
Text HLabel 2500 2300 2    50   BiDi ~ 0
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
P 7150 3050
F 0 "J13" H 7329 3096 50  0000 L CNN
F 1 "DB25_Female" H 7329 3005 50  0000 L CNN
F 2 "Connector_Dsub:DSUB-25_Male_Horizontal_P2.77x2.84mm_EdgePinOffset7.70mm_Housed_MountingHolesOffset9.12mm" H 7150 3050 50  0001 C CNN
F 3 "http://portal.fciconnect.com/Comergent//fci/drawing/c-dd-0024.pdf" H 7150 3050 50  0001 C CNN
F 4 "609-5920-ND" H 7150 3050 50  0001 C CNN "Digikey"
	1    7150 3050
	1    0    0    -1  
$EndComp
Text Notes 8850 2300 0    50   ~ 0
Keyboard
Text Notes 8900 3450 0    50   ~ 0
Mouse
Wire Wire Line
	6850 1850 6300 1850
Wire Wire Line
	6850 4150 6700 4150
Wire Wire Line
	6700 4150 6700 3950
Wire Wire Line
	6700 3950 6850 3950
Wire Wire Line
	6700 3950 6700 3750
Wire Wire Line
	6700 3750 6850 3750
Connection ~ 6700 3950
Wire Wire Line
	6700 3750 6700 3550
Wire Wire Line
	6700 3550 6850 3550
Connection ~ 6700 3750
Wire Wire Line
	6700 3550 6700 3350
Wire Wire Line
	6700 3350 6850 3350
Connection ~ 6700 3550
Wire Wire Line
	6850 3150 6700 3150
Wire Wire Line
	6700 3150 6700 3350
Connection ~ 6700 3350
Wire Wire Line
	6850 2950 6700 2950
Wire Wire Line
	6700 2950 6700 3150
Connection ~ 6700 3150
Wire Wire Line
	6850 2750 6700 2750
Wire Wire Line
	6700 2750 6700 2950
Connection ~ 6700 2950
$Comp
L power:GND #PWR035
U 1 1 5C870121
P 6700 4450
F 0 "#PWR035" H 6700 4200 50  0001 C CNN
F 1 "GND" H 6705 4277 50  0000 C CNN
F 2 "" H 6700 4450 50  0001 C CNN
F 3 "" H 6700 4450 50  0001 C CNN
	1    6700 4450
	1    0    0    -1  
$EndComp
Wire Wire Line
	6700 4150 6700 4450
Connection ~ 6700 4150
Wire Wire Line
	6850 1950 6300 1950
Wire Wire Line
	6850 2050 6300 2050
Wire Wire Line
	6850 2150 6300 2150
Wire Wire Line
	6850 2250 6300 2250
Wire Wire Line
	6850 2350 6300 2350
Wire Wire Line
	6850 2450 6300 2450
Wire Wire Line
	6850 2550 6300 2550
Wire Wire Line
	6850 2650 6300 2650
Wire Wire Line
	6850 2850 6300 2850
Wire Wire Line
	6850 3050 6300 3050
Wire Wire Line
	6850 3250 6300 3250
Wire Wire Line
	6850 3450 6300 3450
Wire Wire Line
	6850 3650 6300 3650
Wire Wire Line
	6850 3850 6300 3850
Wire Wire Line
	6850 4050 6300 4050
Wire Wire Line
	6850 4250 6300 4250
Text Label 6350 1850 0    50   ~ 0
STROBE
Text Label 6350 2050 0    50   ~ 0
D0
Text Label 6350 2250 0    50   ~ 0
D1
Text Label 6350 2450 0    50   ~ 0
D2
Text Label 6350 2650 0    50   ~ 0
D3
Text Label 6350 2850 0    50   ~ 0
D4
Text Label 6350 3050 0    50   ~ 0
D5
Text Label 6350 3250 0    50   ~ 0
D6
Text Label 6350 3450 0    50   ~ 0
D7
Text Label 6350 3850 0    50   ~ 0
BUSY
Text Label 6350 4050 0    50   ~ 0
PE
Text Label 6350 4250 0    50   ~ 0
SEL
Text Label 6350 1950 0    50   ~ 0
AUTOF
Text Label 6350 2150 0    50   ~ 0
ERROR
Text Label 6350 2350 0    50   ~ 0
INIT
Text Label 6350 2550 0    50   ~ 0
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
Text Label 6350 3650 0    50   ~ 0
ACK
Wire Wire Line
	4500 2750 5100 2750
Wire Wire Line
	4500 2850 5100 2850
Text Label 4600 2750 0    50   ~ 0
KB_CLK
Text Label 4600 2850 0    50   ~ 0
MS_CLK
Wire Wire Line
	4500 2950 5100 2950
Text Label 4600 2950 0    50   ~ 0
KB_DATA
Text Label 4600 3050 0    50   ~ 0
MS_DATA
Text HLabel 2500 2400 2    50   Output ~ 0
MCU_TO_IO
$Comp
L power:+5V #PWR033
U 1 1 5C61C941
P 3900 1400
F 0 "#PWR033" H 3900 1250 50  0001 C CNN
F 1 "+5V" H 3915 1573 50  0000 C CNN
F 2 "" H 3900 1400 50  0001 C CNN
F 3 "" H 3900 1400 50  0001 C CNN
	1    3900 1400
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR034
U 1 1 5C61CB48
P 3900 4550
F 0 "#PWR034" H 3900 4300 50  0001 C CNN
F 1 "GND" H 3905 4377 50  0000 C CNN
F 2 "" H 3900 4550 50  0001 C CNN
F 3 "" H 3900 4550 50  0001 C CNN
	1    3900 4550
	1    0    0    -1  
$EndComp
Wire Wire Line
	2500 2300 2050 2300
Wire Wire Line
	2500 2400 2050 2400
Text Label 2100 2400 0    50   ~ 0
UART_RX
Text Label 2100 2300 0    50   ~ 0
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
P 3900 3050
F 0 "U5" H 3260 3096 50  0000 R CNN
F 1 "ATmega48A-PU" H 3260 3005 50  0000 R CNN
F 2 "Package_DIP:DIP-28_W7.62mm" H 3900 3050 50  0001 C CIN
F 3 "http://ww1.microchip.com/downloads/en/DeviceDoc/ATmega48A_88A_168A-Data-Sheet-40002007A.pdf" H 3900 3050 50  0001 C CNN
	1    3900 3050
	1    0    0    -1  
$EndComp
Wire Wire Line
	3900 1550 3900 1450
Wire Wire Line
	4000 1550 4000 1450
Wire Wire Line
	4000 1450 3900 1450
Connection ~ 3900 1450
Wire Wire Line
	3900 1450 3900 1400
Wire Wire Line
	4500 3550 5100 3550
Wire Wire Line
	4500 3650 5100 3650
Text Label 4600 3550 0    50   ~ 0
UART_RX
Text Label 4600 3650 0    50   ~ 0
UART_TX
Wire Wire Line
	4500 3250 5100 3250
Wire Wire Line
	4500 3150 5100 3150
Wire Wire Line
	4500 2150 5100 2150
Wire Wire Line
	4500 2050 5100 2050
Wire Wire Line
	4500 1950 5100 1950
Wire Wire Line
	4500 1850 5100 1850
Wire Wire Line
	4500 3750 5100 3750
Wire Wire Line
	4500 3850 5100 3850
Wire Wire Line
	4500 3950 5100 3950
Wire Wire Line
	4500 4050 5100 4050
Wire Wire Line
	4500 4150 5100 4150
Wire Wire Line
	4500 4250 5100 4250
Wire Wire Line
	4500 2250 5100 2250
Wire Wire Line
	4500 2350 5100 2350
Wire Wire Line
	4500 2450 5100 2450
Wire Wire Line
	4500 2550 5100 2550
Wire Wire Line
	4500 3050 5100 3050
Text Label 4600 1850 0    50   ~ 0
D0
Text Label 4600 1950 0    50   ~ 0
D1
Text Label 4600 2050 0    50   ~ 0
D2
Text Label 4600 2150 0    50   ~ 0
D3
Text Label 4600 2250 0    50   ~ 0
D4
Text Label 4600 2350 0    50   ~ 0
D5
Text Label 4600 2450 0    50   ~ 0
D6
Text Label 4600 2550 0    50   ~ 0
D7
Text Label 4600 3750 0    50   ~ 0
ACK
Text Label 4600 3850 0    50   ~ 0
BUSY
Text Label 4600 3950 0    50   ~ 0
PE
Text Label 4600 4050 0    50   ~ 0
ERROR
Text Label 4600 4150 0    50   ~ 0
AUTOF
Text Label 4600 4250 0    50   ~ 0
STROBE
Text Label 4600 3150 0    50   ~ 0
INIT
Text Label 4600 3250 0    50   ~ 0
SEL
Wire Wire Line
	4500 3350 5100 3350
Text Label 4600 3350 0    50   ~ 0
SELPRIN
Wire Wire Line
	3900 1450 3100 1450
Wire Wire Line
	3100 1450 3100 1850
Wire Wire Line
	3100 1850 3300 1850
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
$EndSCHEMATC
