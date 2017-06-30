require "helix_runtime"

begin
  require "ruby_sql_lexer/native"
rescue LoadError
  warn "Unable to load ruby_sql_lexer/native. Please run `rake build`"
end
