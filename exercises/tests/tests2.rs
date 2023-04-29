// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

pub struct Participant<'a> {
    name: &'a str,
    event: &'a str,
}

impl<'a> Participant<'a> {
    pub fn event_type(&self) -> &str {
        &self.event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let participant = Participant {
            name: "Himanshu",
            event: "GDSC",
        };

        assert_eq!(participant.event_type(), "GDSC");
    }
}
