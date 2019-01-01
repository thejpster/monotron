EESchema Schematic File Version 4
LIBS:monotron-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 2
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text HLabel 8600 3300 2    50   BiDi ~ 0
IO_TO_MCU
$Comp
L Connector:Mini-DIN-6 J11
U 1 1 5C86652F
P 8500 4150
F 0 "J11" H 8500 4517 50  0000 C CNN
F 1 "Mini-DIN-6" H 8500 4426 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Horizontal" H 8500 4150 50  0001 C CNN
F 3 "https://www.te.com/commerce/DocumentDelivery/DDEController?Action=srchrtrv&DocNm=5749180&DocType=Customer+Drawing&DocLang=English" H 8500 4150 50  0001 C CNN
F 4 "5749180-1-ND" H 8500 4150 50  0001 C CNN "Digikey"
	1    8500 4150
	1    0    0    -1  
$EndComp
$Comp
L Connector:Mini-DIN-6 J12
U 1 1 5C866618
P 8500 5300
F 0 "J12" H 8500 5667 50  0000 C CNN
F 1 "Mini-DIN-6" H 8500 5576 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Horizontal" H 8500 5300 50  0001 C CNN
F 3 "https://www.te.com/commerce/DocumentDelivery/DDEController?Action=srchrtrv&DocNm=5749180&DocType=Customer+Drawing&DocLang=English" H 8500 5300 50  0001 C CNN
F 4 "5749180-1-ND" H 8500 5300 50  0001 C CNN "Digikey"
	1    8500 5300
	1    0    0    -1  
$EndComp
$Comp
L Connector:DB25_Female J10
U 1 1 5C866771
P 6500 4300
F 0 "J10" H 6679 4346 50  0000 L CNN
F 1 "DB25_Female" H 6679 4255 50  0000 L CNN
F 2 "Connector_Dsub:DSUB-25_Male_Horizontal_P2.77x2.84mm_EdgePinOffset7.70mm_Housed_MountingHolesOffset9.12mm" H 6500 4300 50  0001 C CNN
F 3 "http://portal.fciconnect.com/Comergent//fci/drawing/c-dd-0024.pdf" H 6500 4300 50  0001 C CNN
F 4 "609-5920-ND" H 6500 4300 50  0001 C CNN "Digikey"
	1    6500 4300
	1    0    0    -1  
$EndComp
Text Notes 8350 3700 0    50   ~ 0
Keyboard
Text Notes 8400 4850 0    50   ~ 0
Mouse
Wire Wire Line
	6200 3100 5650 3100
Wire Wire Line
	6200 5400 6050 5400
Wire Wire Line
	6050 5400 6050 5200
Wire Wire Line
	6050 5200 6200 5200
Wire Wire Line
	6050 5200 6050 5000
Wire Wire Line
	6050 5000 6200 5000
Connection ~ 6050 5200
Wire Wire Line
	6050 5000 6050 4800
Wire Wire Line
	6050 4800 6200 4800
Connection ~ 6050 5000
Wire Wire Line
	6050 4800 6050 4600
Wire Wire Line
	6050 4600 6200 4600
Connection ~ 6050 4800
Wire Wire Line
	6200 4400 6050 4400
Wire Wire Line
	6050 4400 6050 4600
Connection ~ 6050 4600
Wire Wire Line
	6200 4200 6050 4200
Wire Wire Line
	6050 4200 6050 4400
Connection ~ 6050 4400
Wire Wire Line
	6200 4000 6050 4000
Wire Wire Line
	6050 4000 6050 4200
Connection ~ 6050 4200
$Comp
L power:GND #PWR031
U 1 1 5C870121
P 6050 5700
F 0 "#PWR031" H 6050 5450 50  0001 C CNN
F 1 "GND" H 6055 5527 50  0000 C CNN
F 2 "" H 6050 5700 50  0001 C CNN
F 3 "" H 6050 5700 50  0001 C CNN
	1    6050 5700
	1    0    0    -1  
$EndComp
Wire Wire Line
	6050 5400 6050 5700
Connection ~ 6050 5400
Wire Wire Line
	6200 3200 5650 3200
Wire Wire Line
	6200 3300 5650 3300
Wire Wire Line
	6200 3400 5650 3400
Wire Wire Line
	6200 3500 5650 3500
Wire Wire Line
	6200 3600 5650 3600
Wire Wire Line
	6200 3700 5650 3700
Wire Wire Line
	6200 3800 5650 3800
Wire Wire Line
	6200 3900 5650 3900
Wire Wire Line
	6200 4100 5650 4100
Wire Wire Line
	6200 4300 5650 4300
Wire Wire Line
	6200 4500 5650 4500
Wire Wire Line
	6200 4700 5650 4700
Wire Wire Line
	6200 4900 5650 4900
Wire Wire Line
	6200 5100 5650 5100
Wire Wire Line
	6200 5300 5650 5300
Wire Wire Line
	6200 5500 5650 5500
Text Label 5700 3100 0    50   ~ 0
STROBE
Text Label 5700 3300 0    50   ~ 0
D0
Text Label 5700 3500 0    50   ~ 0
D1
Text Label 5700 3700 0    50   ~ 0
D2
Text Label 5700 3900 0    50   ~ 0
D3
Text Label 5700 4100 0    50   ~ 0
D4
Text Label 5700 4300 0    50   ~ 0
D5
Text Label 5700 4500 0    50   ~ 0
D6
Text Label 5700 4700 0    50   ~ 0
D7
Text Label 5700 5100 0    50   ~ 0
BUSY
Text Label 5700 5300 0    50   ~ 0
PE
Text Label 5700 5500 0    50   ~ 0
SEL
Text Label 5700 3200 0    50   ~ 0
AUTOF
Text Label 5700 3400 0    50   ~ 0
ERROR
Text Label 5700 3600 0    50   ~ 0
INIT
Text Label 5700 3800 0    50   ~ 0
SELPRIN
NoConn ~ 8200 4050
NoConn ~ 8200 5200
NoConn ~ 8200 4250
NoConn ~ 8200 5400
Wire Wire Line
	8800 4050 9150 4050
Wire Wire Line
	8800 4250 9150 4250
Wire Wire Line
	8800 5200 9150 5200
Wire Wire Line
	8800 5400 9150 5400
$Comp
L power:GND #PWR035
U 1 1 5C89AC5C
P 8900 5500
F 0 "#PWR035" H 8900 5250 50  0001 C CNN
F 1 "GND" H 8905 5327 50  0000 C CNN
F 2 "" H 8900 5500 50  0001 C CNN
F 3 "" H 8900 5500 50  0001 C CNN
	1    8900 5500
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR034
U 1 1 5C89AC8A
P 8900 4400
F 0 "#PWR034" H 8900 4150 50  0001 C CNN
F 1 "GND" H 8905 4227 50  0000 C CNN
F 2 "" H 8900 4400 50  0001 C CNN
F 3 "" H 8900 4400 50  0001 C CNN
	1    8900 4400
	1    0    0    -1  
$EndComp
Wire Wire Line
	8800 4150 8900 4150
Wire Wire Line
	8900 4150 8900 4400
Wire Wire Line
	8800 5300 8900 5300
Wire Wire Line
	8900 5300 8900 5500
$Comp
L power:+5V #PWR032
U 1 1 5C89CCFE
P 7950 4000
F 0 "#PWR032" H 7950 3850 50  0001 C CNN
F 1 "+5V" H 7965 4173 50  0000 C CNN
F 2 "" H 7950 4000 50  0001 C CNN
F 3 "" H 7950 4000 50  0001 C CNN
	1    7950 4000
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR033
U 1 1 5C89CD2C
P 7950 5150
F 0 "#PWR033" H 7950 5000 50  0001 C CNN
F 1 "+5V" H 7965 5323 50  0000 C CNN
F 2 "" H 7950 5150 50  0001 C CNN
F 3 "" H 7950 5150 50  0001 C CNN
	1    7950 5150
	1    0    0    -1  
$EndComp
Wire Wire Line
	8200 5300 7950 5300
Wire Wire Line
	7950 5300 7950 5150
Wire Wire Line
	8200 4150 7950 4150
Wire Wire Line
	7950 4150 7950 4000
Text Label 8950 4050 0    50   ~ 0
KB_CLK
Text Label 8950 4250 0    50   ~ 0
KB_DATA
Text Label 9000 5200 0    50   ~ 0
MS_CLK
Text Label 9000 5400 0    50   ~ 0
MS_DATA
Text Label 5700 4900 0    50   ~ 0
ACK
Wire Wire Line
	3500 3100 4100 3100
Wire Wire Line
	3500 3200 4100 3200
Text Label 3600 3100 0    50   ~ 0
KB_CLK
Text Label 3600 3200 0    50   ~ 0
MS_CLK
Wire Wire Line
	3500 3300 4100 3300
Text Label 3600 3300 0    50   ~ 0
KB_DATA
Text Label 3600 3400 0    50   ~ 0
MS_DATA
Text HLabel 8600 3400 2    50   Output ~ 0
MCU_TO_IO
$Comp
L power:+5V #PWR027
U 1 1 5C61C941
P 2900 1750
F 0 "#PWR027" H 2900 1600 50  0001 C CNN
F 1 "+5V" H 2915 1923 50  0000 C CNN
F 2 "" H 2900 1750 50  0001 C CNN
F 3 "" H 2900 1750 50  0001 C CNN
	1    2900 1750
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR028
U 1 1 5C61CB48
P 2900 4900
F 0 "#PWR028" H 2900 4650 50  0001 C CNN
F 1 "GND" H 2905 4727 50  0000 C CNN
F 2 "" H 2900 4900 50  0001 C CNN
F 3 "" H 2900 4900 50  0001 C CNN
	1    2900 4900
	1    0    0    -1  
$EndComp
Wire Wire Line
	8600 3300 8150 3300
Wire Wire Line
	8600 3400 8150 3400
Text Label 8200 3400 0    50   ~ 0
UART_RX
Text Label 8200 3300 0    50   ~ 0
UART_TX
$Comp
L power:+5V #PWR029
U 1 1 5C7704F7
P 5150 1900
F 0 "#PWR029" H 5150 1750 50  0001 C CNN
F 1 "+5V" H 5165 2073 50  0000 C CNN
F 2 "" H 5150 1900 50  0001 C CNN
F 3 "" H 5150 1900 50  0001 C CNN
	1    5150 1900
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR030
U 1 1 5C770515
P 5150 2400
F 0 "#PWR030" H 5150 2150 50  0001 C CNN
F 1 "GND" H 5155 2227 50  0000 C CNN
F 2 "" H 5150 2400 50  0001 C CNN
F 3 "" H 5150 2400 50  0001 C CNN
	1    5150 2400
	1    0    0    -1  
$EndComp
$Comp
L Device:C C11
U 1 1 5C77054A
P 4950 2150
F 0 "C11" H 5000 2250 50  0000 L CNN
F 1 "100n" H 5000 2050 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 4988 2000 50  0001 C CNN
F 3 "~" H 4950 2150 50  0001 C CNN
	1    4950 2150
	1    0    0    -1  
$EndComp
$Comp
L Device:C C12
U 1 1 5C77061D
P 5350 2150
F 0 "C12" H 5400 2250 50  0000 L CNN
F 1 "100n" H 5400 2050 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 5388 2000 50  0001 C CNN
F 3 "~" H 5350 2150 50  0001 C CNN
	1    5350 2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	4950 2300 4950 2350
Wire Wire Line
	4950 2350 5150 2350
Wire Wire Line
	5150 2350 5150 2400
Connection ~ 5150 2350
Wire Wire Line
	4950 2000 4950 1950
Wire Wire Line
	4950 1950 5150 1950
Wire Wire Line
	5150 1950 5150 1900
Connection ~ 5150 1950
Wire Wire Line
	5350 2350 5350 2300
Wire Wire Line
	5150 2350 5350 2350
Wire Wire Line
	5350 2000 5350 1950
Wire Wire Line
	5150 1950 5350 1950
$Comp
L MCU_Microchip_ATmega:ATmega48A-PU U3
U 1 1 5C31A32D
P 2900 3400
F 0 "U3" H 2260 3446 50  0000 R CNN
F 1 "ATmega48A-PU" H 2260 3355 50  0000 R CNN
F 2 "Package_DIP:DIP-28_W7.62mm" H 2900 3400 50  0001 C CIN
F 3 "http://ww1.microchip.com/downloads/en/DeviceDoc/ATmega48A_88A_168A-Data-Sheet-40002007A.pdf" H 2900 3400 50  0001 C CNN
	1    2900 3400
	1    0    0    -1  
$EndComp
Wire Wire Line
	2900 1900 2900 1800
Wire Wire Line
	3000 1900 3000 1800
Wire Wire Line
	3000 1800 2900 1800
Connection ~ 2900 1800
Wire Wire Line
	2900 1800 2900 1750
Wire Wire Line
	3500 3900 4100 3900
Wire Wire Line
	3500 4000 4100 4000
Text Label 3600 3900 0    50   ~ 0
UART_RX
Text Label 3600 4000 0    50   ~ 0
UART_TX
Wire Wire Line
	3500 3600 4100 3600
Wire Wire Line
	3500 3500 4100 3500
Wire Wire Line
	3500 2500 4100 2500
Wire Wire Line
	3500 2400 4100 2400
Wire Wire Line
	3500 2300 4100 2300
Wire Wire Line
	3500 2200 4100 2200
Wire Wire Line
	3500 4100 4100 4100
Wire Wire Line
	3500 4200 4100 4200
Wire Wire Line
	3500 4300 4100 4300
Wire Wire Line
	3500 4400 4100 4400
Wire Wire Line
	3500 4500 4100 4500
Wire Wire Line
	3500 4600 4100 4600
Wire Wire Line
	3500 2600 4100 2600
Wire Wire Line
	3500 2700 4100 2700
Wire Wire Line
	3500 2800 4100 2800
Wire Wire Line
	3500 2900 4100 2900
Wire Wire Line
	3500 3400 4100 3400
Text Label 3600 2200 0    50   ~ 0
D0
Text Label 3600 2300 0    50   ~ 0
D1
Text Label 3600 2400 0    50   ~ 0
D2
Text Label 3600 2500 0    50   ~ 0
D3
Text Label 3600 2600 0    50   ~ 0
D4
Text Label 3600 2700 0    50   ~ 0
D5
Text Label 3600 2800 0    50   ~ 0
D6
Text Label 3600 2900 0    50   ~ 0
D7
Text Label 3600 4100 0    50   ~ 0
ACK
Text Label 3600 4200 0    50   ~ 0
BUSY
Text Label 3600 4300 0    50   ~ 0
PE
Text Label 3600 4400 0    50   ~ 0
SEL
Text Label 3600 4500 0    50   ~ 0
STROBE
Text Label 3600 4600 0    50   ~ 0
AUTOF
Text Label 3600 3500 0    50   ~ 0
ERROR
Text Label 3600 3600 0    50   ~ 0
INIT
Wire Wire Line
	3500 3700 4100 3700
Text Label 3600 3700 0    50   ~ 0
SELPRIN
Wire Wire Line
	2900 1800 2100 1800
Wire Wire Line
	2100 1800 2100 2200
Wire Wire Line
	2100 2200 2300 2200
$Comp
L Device:C C13
U 1 1 5C35D035
P 5700 2150
F 0 "C13" H 5750 2250 50  0000 L CNN
F 1 "100n" H 5750 2050 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 5738 2000 50  0001 C CNN
F 3 "~" H 5700 2150 50  0001 C CNN
	1    5700 2150
	1    0    0    -1  
$EndComp
$Comp
L Device:C C14
U 1 1 5C35D061
P 6050 2150
F 0 "C14" H 6100 2250 50  0000 L CNN
F 1 "100n" H 6100 2050 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 6088 2000 50  0001 C CNN
F 3 "~" H 6050 2150 50  0001 C CNN
	1    6050 2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	5350 2350 5700 2350
Wire Wire Line
	6050 2350 6050 2300
Connection ~ 5350 2350
Wire Wire Line
	5700 2300 5700 2350
Connection ~ 5700 2350
Wire Wire Line
	5700 2350 6050 2350
Wire Wire Line
	5350 1950 5700 1950
Wire Wire Line
	5700 1950 5700 2000
Connection ~ 5350 1950
Wire Wire Line
	5700 1950 6050 1950
Wire Wire Line
	6050 1950 6050 2000
Connection ~ 5700 1950
$EndSCHEMATC
