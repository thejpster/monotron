MM_PER_INCH = 25.4;

// ATX specification units are in inches, but we want our DXF in mm
scale([MM_PER_INCH, MM_PER_INCH, 0]) {
	difference() {
		// 6.7" x 6.7" PCB
		translate([6.7 - 9.6, -6.7 ]) square([9.6, 6.7]);
		// Six mounting Holes
		// Top Centre
		translate([0.25, -0.4]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Top Left
		translate([0.25 - 1.8, -0.4]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Top Right
		translate([0.25 + 6.2, -(0.4 + 0.9)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Bottom Middle
		translate([0.25, -(0.4 + 6.1)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Bottom Left
		translate([0.25 - 1.8, -(0.4 + 6.1)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Bottom Right
		translate([0.25 + 6.2, -(0.4 + 6.1)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		// Expansion Slots.
		// The centre of the ISA card (which have components on the CPU/right side) in slot 2
		// is 0.3" left of the top-left mounting hole. Slot 3 is 0.5" to the right of the
		// mounting hole.
		// Slot 1 (far left)
		translate([0.25 - (1.8 + 1.1), -1.6]) square([0.1, 1.6], center=true);
		// Slot 1
		translate([0.25 - (1.8 + 0.3), -1.6]) square([0.1, 1.6], center=true);
		// Slot 1
		translate([0.25 - (1.8 - 0.5), -1.6]) square([0.1, 1.6], center=true);
		// Slot 1 (far right)
		translate([0.25 - (1.8 - 1.3), -1.6]) square([0.1, 1.6], center=true);
	}
}
