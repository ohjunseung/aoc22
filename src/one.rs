pub fn one(list: Vec<u32>) -> u32 {
    let mut tmp = 0u32;
    let mut fattest = 0u32;
    for food in list {
        tmp += food;
        if tmp > fattest {
            fattest = tmp
        }
        if food == 0 {
            tmp = 0;
        }
    }

    fattest
}
