#[test]
fn move_count_one_disk() {
    assert_eq!((1u64 << 1) - 1, 1);
}

#[test]
fn move_count_three_disks() {
    assert_eq!((1u64 << 3) - 1, 7);
}

#[test]
fn move_count_ten_disks() {
    assert_eq!((1u64 << 10) - 1, 1023);
}

#[test]
fn move_count_sixteen_disks() {
    assert_eq!((1u64 << 16) - 1, 65535);
}

#[test]
fn recursive_and_iterative_same_output() {
    fn rec(n: u64, f: u8, t: u8, v: u8, out: &mut Vec<u8>) {
        if n == 0 { return; }
        rec(n-1, f, v, t, out);
        out.extend_from_slice(format!("{} {}\n", f, t).as_bytes());
        rec(n-1, v, t, f, out);
    }
    fn itr(n: u64, f: u8, t: u8, v: u8, out: &mut Vec<u8>) {
        let mut stack = vec![(n, f, t, v)];
        while let Some((cnt, src, dst, hlp)) = stack.pop() {
            if cnt == 0 { continue; }
            else if cnt == 1 {
                out.extend_from_slice(format!("{} {}\n", src, dst).as_bytes());
            } else {
                stack.push((cnt-1, hlp, dst, src));
                stack.push((1, src, dst, hlp));
                stack.push((cnt-1, src, hlp, dst));
            }
        }
    }
    let mut r = Vec::new(); rec(4, 1, 3, 2, &mut r);
    let mut i = Vec::new(); itr(4, 1, 3, 2, &mut i);
    assert_eq!(r, i, "recursive and iterative must produce identical moves");
}
