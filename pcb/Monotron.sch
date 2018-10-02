EESchema Schematic File Version 4
LIBS:Monotron-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Monotron:StellarisLaunchpad U1
U 1 1 5B883CD0
P 6250 3750
F 0 "U1" H 6250 4925 50  0000 C CNN
F 1 "TivaCLaunchpad" H 6250 4834 50  0000 C CNN
F 2 "Monotron:Tiva-C Launchpad Connector" H 6250 3750 50  0001 C CNN
F 3 "" H 6250 3750 50  0001 C CNN
	1    6250 3750
	1    0    0    -1  
$EndComp
$Comp
L Device:R R2
U 1 1 5B883E00
P 2350 2750
F 0 "R2" V 2557 2750 50  0000 C CNN
F 1 "330" V 2466 2750 50  0000 C CNN
F 2 "Resistors_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 2280 2750 50  0001 C CNN
F 3 "~" H 2350 2750 50  0001 C CNN
	1    2350 2750
	1    0    0    -1  
$EndComp
$Comp
L Device:R R1
U 1 1 5B883EA4
P 2750 2750
F 0 "R1" V 2957 2750 50  0000 C CNN
F 1 "330" V 2866 2750 50  0000 C CNN
F 2 "Resistors_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 2680 2750 50  0001 C CNN
F 3 "~" H 2750 2750 50  0001 C CNN
	1    2750 2750
	1    0    0    -1  
$EndComp
$Comp
L Device:R R3
U 1 1 5B883F05
P 1950 2750
F 0 "R3" V 2157 2750 50  0000 C CNN
F 1 "330" V 2066 2750 50  0000 C CNN
F 2 "Resistors_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 1880 2750 50  0001 C CNN
F 3 "~" H 1950 2750 50  0001 C CNN
	1    1950 2750
	1    0    0    -1  
$EndComp
$Comp
L Connector:DB15_Female_HighDensity J1
U 1 1 5B8842EE
P 3200 3850
F 0 "J1" H 3200 4717 50  0000 C CNN
F 1 "DB15_Female_HighDensity" H 3200 4626 50  0000 C CNN
F 2 "Monotron:ICD15S13E4GX00LF" H 2250 4250 50  0001 C CNN
F 3 " ~" H 2250 4250 50  0001 C CNN
	1    3200 3850
	1    0    0    -1  
$EndComp
Wire Wire Line
	5300 4300 5000 4300
$Comp
L Connector:Mini-DIN-6 J2
U 1 1 5BAD1E91
P 6200 1500
F 0 "J2" H 6200 1867 50  0000 C CNN
F 1 "Mini-DIN-6" H 6200 1776 50  0000 C CNN
F 2 "Monotron:5749180-1" H 6200 1500 50  0001 C CNN
F 3 "http://service.powerdynamics.com/ec/Catalog17/Section%2011.pdf" H 6200 1500 50  0001 C CNN
	1    6200 1500
	1    0    0    -1  
$EndComp
$Comp
L Connector:AudioJack3_Ground J3
U 1 1 5BAD2006
P 2650 6150
F 0 "J3" H 2654 6492 50  0000 C CNN
F 1 "AudioJack3_Ground" H 2654 6401 50  0000 C CNN
F 2 "" H 2650 6150 50  0001 C CNN
F 3 "~" H 2650 6150 50  0001 C CNN
	1    2650 6150
	1    0    0    -1  
$EndComp
$Comp
L Connector:DB9_Male J4
U 1 1 5BAD20FF
P 9600 3600
F 0 "J4" H 9780 3646 50  0000 L CNN
F 1 "DB9_Male" H 9780 3555 50  0000 L CNN
F 2 "" H 9600 3600 50  0001 C CNN
F 3 " ~" H 9600 3600 50  0001 C CNN
	1    9600 3600
	1    0    0    -1  
$EndComp
Text GLabel 1700 3550 0    50   Input ~ 0
GND
Text GLabel 5000 3800 0    50   Input ~ 0
GND
Wire Wire Line
	5000 3800 5300 3800
Wire Wire Line
	1950 3850 1950 2900
Text GLabel 1950 2350 1    50   Input ~ 0
Blue
Wire Wire Line
	1950 2350 1950 2600
Text GLabel 8000 4200 2    50   Input ~ 0
Blue
Wire Wire Line
	7200 4200 8000 4200
Wire Wire Line
	1800 3350 1800 3550
Connection ~ 1800 3550
Wire Wire Line
	1800 3550 1700 3550
Wire Wire Line
	1800 3750 1800 3550
Wire Wire Line
	1800 4250 1800 3750
Connection ~ 1800 3750
Wire Wire Line
	1800 4250 2900 4250
Wire Wire Line
	1800 3750 2900 3750
Wire Wire Line
	1800 3350 2900 3350
Wire Wire Line
	1800 3550 2900 3550
Wire Wire Line
	1950 3850 2900 3850
Wire Wire Line
	2350 3650 2350 2900
Wire Wire Line
	2350 3650 2900 3650
Text GLabel 2350 2350 1    50   Input ~ 0
Green
Wire Wire Line
	2350 2600 2350 2350
Text GLabel 5000 4300 0    50   Input ~ 0
Green
Wire Wire Line
	2750 2900 2750 3450
Wire Wire Line
	2750 3450 2900 3450
Text GLabel 2750 2350 1    50   Input ~ 0
Red
Wire Wire Line
	2750 2350 2750 2600
Text GLabel 8000 3800 2    50   Input ~ 0
Red
Wire Wire Line
	7200 3800 8000 3800
Text GLabel 5000 3400 0    50   Input ~ 0
H-Sync
Text GLabel 5000 2900 0    50   Input ~ 0
V-Sync
Text GLabel 3800 4050 2    50   Input ~ 0
V-Sync
Text GLabel 3800 3850 2    50   Input ~ 0
H-Sync
Wire Wire Line
	3800 4050 3500 4050
Wire Wire Line
	5000 2900 5300 2900
Wire Wire Line
	3800 3850 3500 3850
Wire Wire Line
	5000 3400 5300 3400
Text GLabel 6800 1500 2    50   Input ~ 0
GND
Wire Wire Line
	6500 1500 6800 1500
Text GLabel 8000 4700 2    50   Input ~ 0
5.0V
Wire Wire Line
	8000 4700 7200 4700
Text GLabel 5550 1500 0    50   Input ~ 0
5.0V
Wire Wire Line
	5900 1500 5700 1500
Text GLabel 3300 6050 2    50   Input ~ 0
GND
$Comp
L Connector_Generic:Conn_02x05_Counter_Clockwise J5
U 1 1 5BAE3147
P 5300 6150
F 0 "J5" H 5350 6567 50  0000 C CNN
F 1 "Conn_02x05_Counter_Clockwise" H 5350 6476 50  0000 C CNN
F 2 "" H 5300 6150 50  0001 C CNN
F 3 "~" H 5300 6150 50  0001 C CNN
	1    5300 6150
	1    0    0    -1  
$EndComp
Text GLabel 4650 5950 0    50   Input ~ 0
+DATA
Wire Wire Line
	5100 5950 4650 5950
Text GLabel 4650 6050 0    50   Input ~ 0
+CLK
Wire Wire Line
	4650 6050 5100 6050
Text GLabel 4650 6150 0    50   Input ~ 0
Audio
Wire Wire Line
	5100 6150 4650 6150
Text GLabel 3550 7000 2    50   Input ~ 0
Audio
Wire Wire Line
	2850 6150 2950 6150
Wire Wire Line
	2850 6250 2950 6250
Text GLabel 6800 1600 2    50   Input ~ 0
+DATA
Text GLabel 6800 1400 2    50   Input ~ 0
+CLK
Wire Wire Line
	6500 1400 6650 1400
Wire Wire Line
	6500 1600 6650 1600
$Comp
L Device:R R4
U 1 1 5BAE5AAD
P 6200 950
F 0 "R4" V 5993 950 50  0000 C CNN
F 1 "10K" V 6084 950 50  0000 C CNN
F 2 "Resistors_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 6130 950 50  0001 C CNN
F 3 "~" H 6200 950 50  0001 C CNN
	1    6200 950 
	0    1    1    0   
$EndComp
$Comp
L Device:R R5
U 1 1 5BAE5BAD
P 6200 2050
F 0 "R5" V 5993 2050 50  0000 C CNN
F 1 "10K" V 6084 2050 50  0000 C CNN
F 2 "Resistors_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 6130 2050 50  0001 C CNN
F 3 "~" H 6200 2050 50  0001 C CNN
	1    6200 2050
	0    1    1    0   
$EndComp
Wire Wire Line
	6650 1400 6650 950 
Wire Wire Line
	6650 950  6350 950 
Connection ~ 6650 1400
Wire Wire Line
	6650 1400 6800 1400
Connection ~ 6650 1600
Wire Wire Line
	6650 1600 6800 1600
Connection ~ 5700 1500
Wire Wire Line
	5700 1500 5550 1500
Wire Wire Line
	6050 950  5700 950 
Wire Wire Line
	5700 950  5700 1500
Wire Wire Line
	6650 2050 6350 2050
Wire Wire Line
	6650 1600 6650 2050
Wire Wire Line
	6050 2050 5700 2050
Wire Wire Line
	5700 1500 5700 2050
Text GLabel 4650 6250 0    50   Input ~ 0
JoystickUp
Text GLabel 4650 6350 0    50   Input ~ 0
JoystickDown
Text GLabel 6100 5950 2    50   Input ~ 0
JoystickLeft
Text GLabel 6100 6050 2    50   Input ~ 0
JoystickRight
Text GLabel 6100 6150 2    50   Input ~ 0
BInputPaddle
Text GLabel 6100 6250 2    50   Input ~ 0
InputTrigger
Text GLabel 6100 6350 2    50   Input ~ 0
AInputPaddle
Wire Wire Line
	5100 6250 4650 6250
Wire Wire Line
	4650 6350 5100 6350
Wire Wire Line
	5600 5950 6100 5950
Wire Wire Line
	6100 6050 5600 6050
Wire Wire Line
	5600 6150 6100 6150
Wire Wire Line
	6100 6250 5600 6250
Wire Wire Line
	5600 6350 6100 6350
Text GLabel 9100 4000 0    50   Input ~ 0
JoystickUp
Text GLabel 9100 3900 0    50   Input ~ 0
InputTrigger
Text GLabel 9100 3800 0    50   Input ~ 0
JoystickDown
Text GLabel 9100 3700 0    50   Input ~ 0
5.0V
Text GLabel 9100 3600 0    50   Input ~ 0
JoystickLeft
Text GLabel 9100 3500 0    50   Input ~ 0
GND
Text GLabel 9100 3400 0    50   Input ~ 0
JoystickRight
Text GLabel 9100 3300 0    50   Input ~ 0
AInputPaddle
Text GLabel 9100 3200 0    50   Input ~ 0
BInputPaddle
Wire Wire Line
	9300 4000 9100 4000
Wire Wire Line
	9100 3900 9300 3900
Wire Wire Line
	9300 3800 9100 3800
Wire Wire Line
	9100 3700 9300 3700
Wire Wire Line
	9300 3600 9100 3600
Wire Wire Line
	9100 3500 9300 3500
Wire Wire Line
	9300 3400 9100 3400
Wire Wire Line
	9100 3300 9300 3300
Wire Wire Line
	9300 3200 9100 3200
$Comp
L Device:C C1
U 1 1 5BAFCAEA
P 3200 6350
F 0 "C1" H 3315 6396 50  0000 L CNN
F 1 "3.3n" H 3315 6305 50  0000 L CNN
F 2 "Capacitors_THT:C_Disc_D3.4mm_W2.1mm_P2.50mm" H 3238 6200 50  0001 C CNN
F 3 "~" H 3200 6350 50  0001 C CNN
	1    3200 6350
	1    0    0    -1  
$EndComp
Wire Wire Line
	2950 6150 2950 6250
Connection ~ 2950 6250
$Comp
L Device:R R6
U 1 1 5BB04C33
P 3200 6850
F 0 "R6" H 3270 6896 50  0000 L CNN
F 1 "2.4k" H 3270 6805 50  0000 L CNN
F 2 "Resistors_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 3130 6850 50  0001 C CNN
F 3 "~" H 3200 6850 50  0001 C CNN
	1    3200 6850
	1    0    0    -1  
$EndComp
Wire Wire Line
	2850 6050 3200 6050
Wire Wire Line
	3200 6200 3200 6050
Connection ~ 3200 6050
Wire Wire Line
	3200 6050 3300 6050
Wire Wire Line
	3200 6700 3200 6600
Wire Wire Line
	3200 6600 2950 6600
Wire Wire Line
	2950 6250 2950 6600
Connection ~ 3200 6600
Wire Wire Line
	3200 6600 3200 6500
Wire Wire Line
	3200 7000 3550 7000
$EndSCHEMATC
