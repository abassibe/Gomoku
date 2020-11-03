fn test_bitvec_shift_left() {
    let mut bitvec = bitvec![1; SIZE];
    let by = SHIFT_BY;

    let len = bitvec.len();
    if by == 0 {
        return;
    }
    assert!(by < len, "Cannot shift a slice by more than its length: {} exceeds {}", by, len);
    unsafe {
        bitvec.copy_within_unchecked(by .., 0);
    }
    let trunc = len - by;
    bitvec[trunc ..].set_all(false);

    // println!("{}", bitvec);
}

fn test_bitvec_shift_right() {
    let mut bitvec = bitvec![1; SIZE];
    let by = SHIFT_BY;

    let len = bitvec.len();
    if by == 0 {
        return;
    }
    assert!(by < len, "Cannot shift a slice by more than its length: {} exceeds {}", by, len);
    let trunc = len - by;
    unsafe {
        bitvec.copy_within_unchecked(.. trunc, by);
    }
    bitvec[.. by].set_all(false);

    // println!("{}", bitvec);
}