#[cfg(test)]
#[macro_use]
macro_rules! assert_vec_eq {
    ($left:ident, $right:ident) => {
        if $left.len() != $right.len() {
            println!("{:?} but expected {:?}", $left, $right);
            panic!();
        }

        for x in &$left {
            let mut found = false;

            for y in &$right {
                if x == y {
                    found = true;
                    break;
                }
            }

            if !found {
                println!("{:?} but expected {:?}", $left, $right);
                panic!();
            }
        }

        for x in &$right {
            let mut found = false;

            for y in &$left {
                if x == y {
                    found = true;
                    break;
                }
            }

            if !found {
                println!("{:?} but expected {:?}", $left, $right);
                panic!();
            }
        }
    };
}
