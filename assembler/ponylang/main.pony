use "files"
use "lexer"

actor Main
  new create(env: Env) =>
    let program: String = try env.args(0)?.string() else "" end
    let first_arg: String = try env.args(1)?.string() else "" end

    if env.args.size() != 2 then
      env.out.print("usage: " + program + " <source.asm>")
      env.exitcode(1)
      return
    end

    let fp: FilePath = FilePath.create(FileAuth(env.root), first_arg)

    match OpenFile(fp)
    | let file: File =>
      var source: Source ref = Source(file.read_string(8192))
      var lexer: Lexer = Lexer.create(env.out, source)

      try
        lexer.scan()?
      else
        env.exitcode(1)
        return
      end

      env.out.print("Opened '" + file.path.path + "'")
    else
      env.out.print("Error opening file '" + fp.path + "'")
    end
