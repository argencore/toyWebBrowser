use dom::*;

/// struct for holding the input element(HTML file) and
/// an index within it
struct Parser {
    position: usize,
    input: String,
}

impl Parser{
    /** fn next_char
        # Summary
        The function peeks at the current character without
        consuming it
        # Arguments

        * `&self` - Parser

        # Outputs

        * `char` - the character peeked at
     */
    fn next_char(&self) -> char{
        self.input[self.position ..]
        .chars()
        .next()
        .expect("error: could not get next char at 
        position: {} in fn next_char", self.position)
    }

    /** fn starts_with
        # Summary
        The function checks if the next characters form
        a given string
        # Arguments

        * `&self` - Parser
        * `s` - the string to match on

        # Outputs

        * `bool` - if a match was found or not
     */
    fn starts_with(&self, s: &str) -> bool{
        self.input[self.position ..]
        .starts_with(s) // String method starts_with used here
    }

    /** fn eof
        # Summary
        The function checks if all input has
        been consumed(is at end of file)
        # Arguments

        * `&self` - Parser

        # Output

        * `bool` - if at end of input
     */
    fn eof(&self) -> bool{
        self.position >= self.input
                         .len()
    }

    /** fn consume_char
        # Summary
        The function consumes a character and 
        returns it to the caller
        # Arguments

        * `&mut self` - Parser

        # Outputs

        * `char` - the character consumed
     */
    fn consume_char(&mut self) -> char{
        //get an iterator over the input string
        let mut iter = self.input[self.position ..]
                       .char_indices();
        let (_, cur_char) = iter.next()
                            .expect("could not get cur_char");

        //update the position in the parser
        let (next_pos, _) = iter.next().unwrap_or((1,' '));
        self.position += next_pos;

        //return the current char
        cur_char

    }

    /** fn consume_while
        # Summary
        The function consumes a string of characters
        that meet a given condition
        # Arguments

        * `&mut self` - Parser
        * `test` - F where F is the condition to 
        meet(a function or closure mapping Char to bool)

        # Output

        * `String` - the matching string that is consumed

     */
    fn consume_while<F>(&mut self, test: F) -> String
    where F: Fn(Char) -> bool{
        let mut result = String::new();
        while !self.eof() && test(self.next_char()){
            result.push(self.consume_char());
        }
        result
    }

    /** fn consume_whitespace
        # Summary
        The function consumes characters while the characters are whitespace
        # Arguments

        * `&mut self` - Parser
     */
    fn consume_whitespace(&mut self){
        self.consume_while(CharExt::is_whitespace);
    }

    /** fn parse_tag_name
        # Summary
        The function parses a tag by consumeing characters that are 
        in the set (a-z),(A-Z),(0-9)
        # Arguments

        * `&mut self` - Parser

        # Output

        * `String` - the tag that was parsed
     */
    fn parse_tag_name(&mut self) -> String{
        self.consume_while(|c| match c{
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }

    /** fn parse_node
        # Summary
        The function checks for the indicator of a tag and parses
        it as an elements anything else gets parsed as text since those are 
        currently the only two options
        # Arguments

        * `&mut self` - Parser

        #Output

        * `dom::Node` - a node in the dom tree
     */
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char(){
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    /** fn parse_text
        # Summary
        The function parses the text by consuming it untill
        a tag is hit

        # Arguments

        * `&mut self` - Parser

        # Outputs

        * `dom::Node` - a node in the dom tree
     */
    fn parse_text(&mut self) -> dom::Node{
        dom::text(self.consume_while(|c| c != '<'))
    }    

    /** fn parse_element
        # Summary
        The function parses an element by first checking there
        is a valid tag then grabing the tag name and attributes 
        Then the closeing tag is taken and the children are parsed
        followed by a closing tag
        # Arguments

        * `&mut self` - Parser

        # Output

        * `dom::Node` - an element node with children
     */
    fn parse_element(&mut self) -> dom::Node{
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert!(self.consume_char() == '>');

        //contents
        let children = self.parse_nodes();

        //closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::elem(tag_name, attributes, children)

    }
    
}
