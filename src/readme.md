# Project 1
## Xiangning Gao
### 998547853


**CharStream**
main purpose: 
* Char Stream reads the file name and put the whole input including everything and put into a character vector named contents.
functions:
* Curr_pos is used to track the current position of the char, it moves forward, conumes the character and never go back, end_pos is used to track every word's.
* CharStream: opens files.
* from_str: initiates the charstream from a string
* more_available: checks if there's more character available.
* peek_next_char: sees next character without consuming it depending on current curr_pos position.
* peek_ahead_char: sees characters ahead by k position from the curr_pos position.
* get_next_char: moves the curr_pos forward.

**Scanner**
main purpose: 
* Scanner calls charStream build runs the string from charStream and output one token at a time.
functions:
* skip_spaces moves to the next character which omits indentations, backspaces etc.
* handle_identifier reads character, loop if it is upon the character rule. updates global char_pos if successful looping.
    otherwise return the starting current position token as identifier.
* handle_number does the same thing to numbers, but if sees 0 or 1 '.', it will determine its a float. sees more than 1 will be invalid.
* handle_operator sees single char operators except single '!'. sees next char and push together if valid, for example"==", ">=" except for "=!" which is invalid.
* get_next_token: generate a token and consumes it. In get_next_token it checks the starting char of the current char, then put it into proper handles, generates invalid token if current char doesn't fit any of the handles. returns None if no token generate
    

**Parser**
main purpose: 
  * Parser takes in file name and calls scanner, builds a vector of tokens with proper token type. The current token position(mut pos: usize) which is being analyzing currenly is passed down to every function. The current token position is updated in every function block locally when done dealing with a token. Therefore we can always access the passed or future token from the current token position. If a function fails to analyze a token, it will return None. So the upper recursive function can track if it is successtul. If a function successfully analyzed a token, it will return the next token position which is waiting for analyzation in vector. Also, if there's a typed wording comparison, for example: "=", "while", "if" appears, then the current charactor position needs to be mannually updated right away.
functions:
  * parse: runs get_next_tokens in a loop and generates the vector of tokens.
  * check: calls parse and runs the type checking.
  * write_html: output to a xhtml file with propor colors of each token
  * check_program: 
            loops the check_declaration move to next unanalyzed token position. If the token fails for declaration check then the loop breaks and the token position is passed to the next function.
            calls check_main_declatation, does the same checking. the program fails if it doesn't have a main(return None)
            Then if there is a main, check_function_definition. does the same thing to check_declaration, It is Ok check_function_definition fails because it is optional
 * check_declaration: Put the current token into declaration check and see if it returns a position (RECALL: successful checking will move the position to the next token that is waiting for analyzation. failing of checking will return None instead of Some position). If moved then take the current token position and see if the current token will pass the check_variable_declaration test.a new position is returned if successful. Then set the last token to be Variable. Otherwise pass the current token into check_functionn_declaration test and the previous token's type will be updated as function if passed.
 
 * check_main_declaration: check if the first token is Void, second is Main, third is (, then the forth is ) or block. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
 * check_function_definition: check if the first token is declaration_type, second is parameter_block, third is block. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
 * check_declaration_type: check if the first token is data type, second is identifier. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
 * check_variable_declaration: check if the first token is "=" or ";".  
    "=": check if next token is constant, ";": can be itself alone.
    Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_function_declaration: check if the first token is parameter block, then see if it follows by a ";". Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_block: must start with "{". None if no "{". Then loop in check_declaration check for the second token, loop ends if fails and pass the next token to the next loop for check_statement. check_statement does the same thing then pass the next token to check_function_definintion which does the same thing too. Then check if it ends with a "}".
* check_parameter_block: see if it starts with a "(", if the next one is a parameter, loop the rest see if the next one starts with ",". If there's a ',' then there must be a parameter. loop breaks if not, no parameter is ok too. then if there's a parameter or ) follows. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_data_type: see if the token is a float or integer. Return None if it is neither.
* check_statement: see if it contains either assignment, while loop, if statement, return statement, or expressionn. And for expression it must end with a ";". Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_parameter: see if the current token is a data type, and if the next one is an identifier. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_integer_type: see if the first token context is "unsigned". if not then check if it is "char" | "short" | "int" | "long". If it is "unsigned" then see if the following token is "char" | "short" | "int" | "long". Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_float_type: see if the token is either float or double and return the next token's position.
* check_assignment: check if it is an identifier, followed by a "=", then loop see if the current_pos+2 token is "=": breaks the loop if it is not. loop in see if the next one is identifier again. Then check for expression, Then check for ";". Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_while_loop: first see if it starts with the word "while", then if it follows by a "(" then ")" or expression in the middle. Then check for block. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_if_statement: does the same thing as while_loop. starts with "if" instead of "while".
* check_return_statement: see if first token is "return", and see if there's an expression. return None if no expression. Then check for ";".
* check_expression: see if the first one is simple_expression, if it is then check if next one is relation_operator, if it is then check if next one is a simple_expression. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_simple_expression: see if the first one is term, if it is then if next one is add operator, if it is then if next one is term again. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.
* check_term: see if first one is factor, if it is then if next one is multiplication operator, if it is then if next one is a factor. Return None if fails at every step or no token is found. Return the updated position for the next waiting token if all succeed.

* check_factor: see if the first token is a "(", if it is then check if next one is expression and follows by a ")".
                         otherwise see if the first token is constant
                         otherwise see if the first token is an identifier. Then see if the next one is expressionn, if there's an expression then check loops if the next one is a ",", then see if the next token after "," is an expression again. loop ends if doesn't see ',' after expression. Then see if it ends with a ")". is_function variable is used here to track if it is a function, last_pos takes the last token's position, these two will use to update the last token(identifier)'s token type. If there's a "(" after identifier, then is_function = true to update the token when succeed.
* check_relation_operator: see if the token's text falls into either relational operators. Return updated next token's position if successful, None otherwise.
* check_add_operator and check_mult_operators does the same thing for add and multiplication operators.


Problems with the algorithm: Since I am using Option for every method, it is impossible to track which line number the error will occur. I am thinking about using enum to have 2 options that can store token or something as return if I got more time.
