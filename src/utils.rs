use tree_sitter::Node;

pub trait GetStringFromNode {
    fn get_string(self: &Self, code: &String) -> String;
}

impl<'a> GetStringFromNode for Node<'a> {
    fn get_string(self: &Self, code: &String) -> String {
        code.chars()
            .skip(self.start_byte())
            .take(self.end_byte() - self.start_byte())
            .collect()
    }
}
