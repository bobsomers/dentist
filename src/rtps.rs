use super::dds;

struct Reader<T> {
    remove_me: T // TODO: remove me, satisfies the compiler
}

impl<T> dds::DataReader<T> for Reader<T> {
    // TODO
}

struct Writer<T> {
    remove_me: T // TODO: remove me, satisfies the compiler
}

impl<T> dds::DataWriter<T> for Writer<T> {
    // TODO
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
