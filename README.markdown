```
bundle
rake build
bundle console

>> RubySqlLexer.sanitize_string('SeleCT * from users where users.id = 42')
=> "SELECT * FROM users WHERE users.id = ?"

>> RubySqlLexer.lex('select * from `users` inner join user_orgs on users.id = user_orgs.user_id where users.id = 42')
=> ["SELECT", " ", "*", " ", "FROM", " ", "`users`", " ", "INNER", " ", "JOIN", " ", "user_orgs", " ", "ON", " ", "users", ".", "id", " ", "=", " ", "user_orgs", ".", "user_id", " ", "WHERE", " ", "users", ".", "id", " ", "=", " ", "?"]
```
