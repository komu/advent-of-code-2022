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
