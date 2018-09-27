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
F 2 "" H 2250 4250 50  0001 C CNN
F 3 " ~" H 2250 4250 50  0001 C CNN
	1    3200 3850
	1    0    0    -1  
$EndComp
Wire Wire Line
	5300 4300 5000 4300
$Comp
L Connector:Mini-DIN-6 J2
U 1 1 5BAD1E91
P 2700 6450
F 0 "J2" H 2700 6817 50  0000 C CNN
F 1 "Mini-DIN-6" H 2700 6726 50  0000 C CNN
F 2 "" H 2700 6450 50  0001 C CNN
F 3 "http://service.powerdynamics.com/ec/Catalog17/Section%2011.pdf" H 2700 6450 50  0001 C CNN
	1    2700 6450
	1    0    0    -1  
$EndComp
$Comp
L Connector:AudioJack3_Ground J3
U 1 1 5BAD2006
P 4050 6400
F 0 "J3" H 4054 6742 50  0000 C CNN
F 1 "AudioJack3_Ground" H 4054 6651 50  0000 C CNN
F 2 "" H 4050 6400 50  0001 C CNN
F 3 "~" H 4050 6400 50  0001 C CNN
	1    4050 6400
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
$EndSCHEMATC
