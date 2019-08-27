difference() {
	// PCB
	translate([0, -6.7*25.4]) square([6.7*25.4,6.7*25.4]);
	// Mounting Holes
	translate([0.25*25.4,            -0.4*25.4]) circle(d=3.28, $fn=20);
	translate([0.25*25.4 + 6.2*25.4, -0.4*25.4 - 0.9*25.4]) circle(d=3.28, $fn=20);
	translate([0.25*25.4,            -0.4*25.4 - 6.1*25.4]) circle(d=3.28, $fn=20);
	translate([0.25*25.4 + 6.2*25.4, -0.4*25.4 - 6.1*25.4]) circle(d=3.28, $fn=20);
}
