trait TokenType
  fun string(): String

primitive AT is TokenType
  fun string(): String => "AT"
  fun apply(): U32 => '@'

primitive DEST is TokenType
  fun string(): String => "DEST"

primitive EOF is TokenType
  fun string(): String => "EOF"
  fun apply(): U32 => 0

primitive NEWLINE is TokenType
  fun string(): String => "NEWLINE"
  fun apply(): U32 => '\n'

primitive NUM is TokenType
  fun string(): String => "NUMBER"
  fun apply(): U32 => '\n'

primitive SLASH is TokenType
  fun string(): String => "SLASH"
  fun apply(): U32 => '/'

class Token
  let data: String val
  let token_type: TokenType val

  new create(data': String val, token_type': TokenType val) =>
    data = data'
    token_type = token_type'

  new from_rune(rune: U32, token_type': TokenType val) =>
    data = recover val String.from_utf32(rune) end
    token_type = token_type'

  fun string(): String =>
    "TokenType." + token_type.string() + ": " + data
