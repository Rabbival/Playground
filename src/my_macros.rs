#[macro_export]
macro_rules! collect_all {
    () => {
        all::<Self>().collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! read_variant {
    ($reader:expr, $variant:path) => {
        $reader.read().filter_map(|event| {
            if let $variant(value) = event {
                Some(value)
            } else {
                None
            }
        })
    };
}

#[macro_export]
macro_rules! return_if_at_limit {
    ($iterable:expr, $max_count:expr) => {
        if $iterable.into_iter().count() >= $max_count {
            return;
        }
    };
}

#[macro_export]
macro_rules! single_else_return {
    ($query:expr) => {
        match $query.get_single() {
            Ok(item) => item,
            Err(error) => {
                print_error(
                    format!("error getting single {:?}: {}", $query, error),
                    vec![LogCategory::Crucial],
                );
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! single_mut_else_return {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(item) => item,
            Err(error) => {
                print_error(
                    format!("error getting single mut {:?}: {}", $query, error),
                    vec![LogCategory::Crucial],
                );
                return;
            }
        }
    };
}
