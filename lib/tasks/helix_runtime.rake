require 'helix_runtime/build_task'

HelixRuntime::BuildTask.new("ruby_sql_lexer")

task :default => :build
