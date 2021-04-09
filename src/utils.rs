macro_rules! segment {
    (<$x:ident>) => {
        Rule::NonTerminal(stringify!($x).into())
    };

    ($x:expr) => {
        Rule::Terminal($x.into())
    };

    ($x:tt) => {
        Rule::Terminal(stringify!($x).into())
    };

    // pushes segments to a vec instead of returning it
    (@push_or $list:ident $x:tt) => {
        $list.push(segment!($x));
    };

    (@push_or $list:ident <$x:ident>) => {
        $list.push(segment!(<$x>));
    };

    // pushes segments to a vec and recurses over the tail
    // these cases only handle patterns that have the `|` operator
    (@push_or $list:ident $x:tt | $($tail:tt)+) => {
        $list.push(segment!($x));
        segment!(@push_or $list $($tail)+);
    };

    (@push_or $list:ident <$x:ident> | $($tail:tt)+) => {
        $list.push(segment!(<$x>));
        segment!(@push_or $list $($tail)+);
    };

    // pushes segments to a vec and recurses over the tail
    // theses cases only handle the patterns that have the
    // and operator which is just space
    (@push_or $list:ident $x:tt $($tail:tt)+) => {
        $list.push(expression!($x $($tail)+));
    };

    (@push_or $list:ident <$x:ident> $($tail:tt)+) => {
        $list.push(expression!(<$x> $($tail)+));
    };
}

macro_rules! expression {
    ($x:tt | $($tail:tt)+) => {
        {
            let mut _segments = vec![segment!($x)];
            segment!(@push_or _segments $($tail)*);
            Rule::Or(_segments)
        }
    };

    (<$x:ident> | $($tail:tt)+) => {
        {
            let mut _segments = vec![segment!(<$x>)];
            segment!(@push_or _segments $($tail)*);
            Rule::Or(_segments)
        }
    };

    (<$x:ident> $($tail:tt)+) => {
        {
            let mut _segments = vec![segment!(<$x>)];
            segment!(@push_or _segments $($tail)*);
            Rule::And(_segments)
        }
    };

    // if we get here then we only have one segment so we parse just that
    ($x:expr) => {
        {
            let mut _segments = vec![segment!($x)];
            Rule::Or(_segments)
        }
    };

}

macro_rules! rule {
    (<$group:ident> ::= $($tail:tt)*) => {
        {
            (stringify!($group).to_string(), expression!($($tail)*))
        }
    };
}
