* pub,pub(crate)の違い -> crate単位でglobal(mod でimportされる間は使える).一方完全に別moduleからのimportはできない

## TODO:
* ~~storageへの書き込みは &[u8] にしたいけど、ちょっとlifetimeの引き回しがむずそう~~
* マルチスレッド環境の考慮
