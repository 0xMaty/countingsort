// Time complexity: O(N+k)
pub fn sort(a: &[u32], b: &mut [u32], c: &mut [u32]) {
    for &i in a {
        c[i as usize] += 1;
    }

    for i in 1..c.len() {
        c[i] = c[i] + c[i - 1];
    }

    for i in 0..a.len() {
        b[(c[a[i] as usize] - 1) as usize] = a[i];
    }
}
