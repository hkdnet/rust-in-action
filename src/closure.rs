#[cfg(test)]
mod test {
    fn is_fn<T: Fn(i32) -> Vec<i32>>(_: &T) {}
    fn is_fn_mut<T: FnMut(i32) -> Vec<i32>>(_: &T) {}
    fn is_fn_once<T: FnOnce(i32) -> Vec<i32>>(_: &T) {}

    #[test]
    fn test_is_fn() {
        let a = 1;
        is_fn(&|_| vec![]);
        is_fn(&|_| {
            println!("{}", a);
            vec![]
        });
        //        let mut b = 2;
        //        is_fn(&|f| {
        //            b = f;
        //            vec![]
        //        });
        //        let mut c = vec![];
        //        is_fn(&|f| {
        //            c.push(f);
        //            c
        //        })
    }

    #[test]
    fn test_is_fn_mut() {
        let a = 1;
        is_fn_mut(&|_| vec![]);
        is_fn_mut(&|_| {
            println!("{}", a);
            vec![]
        });
        let mut b = 2;
        is_fn_mut(&|f| {
            b = f;
            vec![]
        });
        //        let mut c = vec![];
        //        is_fn_mut(&|f| {
        //            c.push(f);
        //            c
        //        })
    }

    #[test]
    fn test_is_fn_once() {
        let a = 1;
        is_fn_once(&|_| vec![]);
        is_fn_once(&|_| {
            println!("{}", a);
            vec![]
        });
        let mut b = 2;
        is_fn_once(&|f| {
            b = f;
            vec![]
        });
        let mut c = vec![];
        is_fn_once(&|f| {
            c.push(f);
            c
        })
    }

    #[test]
    fn test_foo() {
        let mut a = 1;
        // memo: この mut は外せない
        let mut f = |x| a = x;

        f(2);
        assert_eq!(a, 2);

        // memo: f(2) の時点で a を mutable borrow しているので呼び出せない
        // f(3);
    }
}
