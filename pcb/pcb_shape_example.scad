difference() {
	// PCB
	square([172.72,142.24],center=true);
	// Mounting Holes
	translate([158.75/2, 101.6/2]) circle(d=3.28, $fn=20);
	translate([-158.75/2, 101.6/2]) circle(d=3.28, $fn=20);
	translate([158.75/2, -101.6/2]) circle(d=3.28, $fn=20);
	translate([-158.75/2, -101.6/2]) circle(d=3.28, $fn=20);
}
