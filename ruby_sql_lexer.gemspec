# encoding: utf-8

Gem::Specification.new do |s|
  s.name = 'ruby_sql_lexer'
  s.version = '1.0.0'
  s.authors = ['Ben Holley']
  s.summary = 'A SQL Lexer'
  s.files = Dir['{lib/**/*,[A-Z]*}']
  s.require_path = 'lib'
  s.add_dependency 'helix_runtime'
end
