class Source is Iterator[U32]
  let _string: String box
  var _i: USize

  new create(string: String box) =>
    _string = string
    _i = 0

  fun has_next(): Bool =>
    _i < _string.size()

  fun peek(): U32 =>
    (let rune, let len) = try _string.utf32(_i.isize())? else (0, 0) end
    rune

  fun ref next(): U32 =>
    (let rune, let len) = try _string.utf32(_i.isize())? else (0, 0) end
    _i = _i + len.usize()
    rune


class Lexer
  let out: OutStream
  var source: Source
  var rune: U32 = 0
  var peek: U32 = 0

  new create(out': OutStream, source': Source) =>
    out = out'
    source = source'
    next_rune()

  fun ref next_token(): (Token | None)? =>
    var token: (Token | None) = None

    skip_whitespace()
    skip_comment()

    match rune
    | AT() =>
      token = Token.from_rune(rune, AT)

    | NEWLINE() =>
      token = Token.from_rune(rune, NEWLINE)

    | 'A' =>
      match next_rune()
      | 'M' =>

        match next_rune()
        | 'D' =>

          if next_rune() != '=' then
            out.print("Invalid dest: AMD" + String.from_utf32(peek))
            error
          end
          token = Token.create("AMD", DEST)
        | '=' =>

          token = Token.create("AM", DEST)
        end

      | 'D' =>

        if next_rune() != '=' then
          out.print("Invalid dest: AD" + String.from_utf32(peek))
          error
        end
        token = Token.create("AD", DEST)
      else
        out.print("Invalid dest: A" + String.from_utf32(rune))
        error
      end

    | 'D' =>

        if next_rune() != '=' then
          out.print("Invalid dest: D" + String.from_utf32(peek))
          error
        end
        token = Token.create("D", DEST)

    | let rune': U32 if (rune >= '1') and (rune <= '9') =>

      var data: String = String.from_utf32(rune')
      while (peek >= '0') and (peek <= '9') do
        next_rune()
        data = data + String.from_utf32(rune')
      end
      token = Token.create(data, NUM)
    else
      out.print("Unexpected token " + String.from_utf32(rune))
      error
    end

    next_rune()
    token

  fun ref skip_comment() =>
    match rune
    | SLASH() =>
      if peek == SLASH() then
        while rune != NEWLINE() do
          next_rune()
        end
      end
    end

  fun ref skip_whitespace() =>
    while (rune == ' ') or (rune == '\t') or (rune == '\r') do
      next_rune()
    end

  fun ref next_rune(): U32 =>
    rune = source.next()
    peek = source.peek()
    rune

  fun ref scan()? =>
    while rune != 0 do
      //out.print(rune.string() + ": '" + recover val String.from_utf32(rune) end + "'")
      match next_token()?
      | let token: Token => out.print(token.string())
      end
    end
