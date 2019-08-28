MM_PER_INCH = 25.4;

// ATX specification units are in inches, but we want our DXF in mm
scale([MM_PER_INCH, MM_PER_INCH, 0]) {
	difference() {
		// 6.7" x 6.7" PCB
		translate([6.7 - 9.6, -6.7 ]) square([9.6 - 6.7, 6.7]);
		// Two mounting Holes
		translate([0.25 - 1.8, -0.4]) circle(d=3.28/MM_PER_INCH, $fn=20);
		translate([0.25 - 1.8, -(0.4 + 6.1)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Expansion Slots.
		// The pins are centered on the left and right hand edges of this rectangle.
		// These pins can then be fitted with an ISA card-edge connector,
		// or you can jam the card edge into a 0.1" pin header, with the pins on
		// either side of a 1.6mm PCB.
		translate([0.25 - (2.2 - 1.6), -2.5]) square([0.2, 1.6]);
		translate([0.25 - (2.2 - 0.8), -2.5]) square([0.2, 1.6]);
		translate([0.25 - 2.2, -2.5]) square([0.2, 1.6]);
		translate([0.25 - (2.2 + 0.8), -2.5]) square([0.2, 1.6]);
	}
}
