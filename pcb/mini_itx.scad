MM_PER_INCH = 25.4;

// ATX specification units are in inches, but we want our DXF in mm
scale([MM_PER_INCH, MM_PER_INCH, 0]) {
	difference() {
		// 6.7" x 6.7" PCB
		translate([0, -6.7 ]) square([6.7,6.7]);
		// Four mounting Holes
		translate([0.25, -0.4]) circle(d=3.28/MM_PER_INCH, $fn=20);
		translate([0.25 + 6.2, -(0.4 + 0.9)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		translate([0.25, -(0.4 + 6.1)]) circle(d=3.28/MM_PER_INCH, $fn=20);
		translate([0.25 +6.2, -(0.4 + 6.1)]) circle(d=3.28/MM_PER_INCH, $fn=20);
	}
}
