use float_cmp::approx_eq;

/// Finds which knot span a certain u value is in
///
/// This algorithm is ripped right from the NURBS book
///
/// * `degree`	- The polynomial degree of the basis functions
/// * `knots`	- A vector containing the u value of each knot
/// * 'u'		- The u value
fn find_span(degree: u8, knots: Vec<f32>, u: f32) -> usize {
	let n = knots.len();

	// Cover the edge case at the end
	if approx_eq!(f32, u, knots[n + 1]) {
		return n
	}

	// Binary search to find it
	let mut low = degree as usize;
	let mut high = n + 1;
	let mut mid = (low + high) / 2;

	while u < knots[mid] || u >= knots[mid + 1] {
		if u < knots[mid] {
			high = mid;
		} else {
			low = mid;
		}

		mid = (low + high) / 2;
	}

	mid
}
