use std::str::FromStr;

pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr<Err = anyhow::Error>,
{
    input.lines().map(|s| s.parse::<T>().unwrap())
}

pub fn mut_refs<T>(vs: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert_ne!(i, j);

    if i < j {
        let (l, r) = vs.split_at_mut(j);
        (&mut l[i], &mut r[0])
    } else {
        let (l, r) = vs.split_at_mut(i);
        (&mut r[0], &mut l[j])
    }
}

/// Given three distinct indices to a mutable slice, returns the mutable references to it.
pub fn mut_refs3<T>(vs: &mut [T], i: usize, j: usize, k: usize) -> (&mut T, &mut T, &mut T) {
    assert_ne!(i, j);
    assert_ne!(i, k);
    assert_ne!(j, k);
    assert!(i <= vs.len());
    assert!(j <= vs.len());
    assert!(k <= vs.len());

    let p = vs.as_mut_ptr();
    unsafe { (&mut *p.add(i), &mut *p.add(j), &mut *p.add(k)) }
}
